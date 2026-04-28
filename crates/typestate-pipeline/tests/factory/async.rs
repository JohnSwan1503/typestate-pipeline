//! Async setters (`#[field(setter = …, async_fn)]`) and async finalize
//! (`#[factory(finalize_async(...))]`) — both standalone and Pipeline-integrated.
//!
//! Also demonstrates Q3: finalize used inside a `#[transitions]` body, which
//! is the most flexible way to fold finalization into a Pipeline chain when
//! the work depends on additional context or other inputs.

use core::fmt;

use typestate_pipeline::{
    pipelined, transitions, Pipeline, Resolved, TypestateFactory,
};

// ===========================================================================
// Async setter — standalone
// ===========================================================================

#[derive(Debug)]
enum BadInput {
    Empty,
}

impl fmt::Display for BadInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("empty")
    }
}
impl std::error::Error for BadInput {}

#[derive(TypestateFactory)]
#[factory(error = BadInput)]
struct UserProfile {
    #[field(required, setter = normalize_name_async, async_fn)]
    name: String,
    #[field(required, setter = validate_email_async, async_fn, fallible)]
    email: String,
}

async fn normalize_name_async(value: String) -> String {
    // Simulate an async normalization step (e.g., consulting an i18n service).
    tokio::task::yield_now().await;
    value.trim().to_owned()
}

async fn validate_email_async(value: String) -> Result<String, BadInput> {
    tokio::task::yield_now().await;
    if value.is_empty() {
        Err(BadInput::Empty)
    } else {
        Ok(value.to_lowercase())
    }
}

#[tokio::test]
async fn standalone_async_setter_non_fallible() {
    let bag = UserProfileFactory::new()
        .name("  Alice  ".to_owned()) // async fn, returns Bag directly
        .await;

    // Now apply the fallible async setter.
    let bag = bag
        .email("Alice@Example.COM".to_owned()) // async fn returning Result<Bag, BadInput>
        .await
        .expect("non-empty");

    let user = bag.finalize();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
}

#[tokio::test]
async fn standalone_async_setter_fallible_failure() {
    let bag = UserProfileFactory::new()
        .name("Bob".to_owned())
        .await;
    let result = bag.email(String::new()).await;
    match result {
        Err(BadInput::Empty) => {}
        Ok(_) => panic!("expected BadInput::Empty"),
    }
}

// ===========================================================================
// Async finalize — standalone
// ===========================================================================

#[derive(Debug, PartialEq, Eq)]
struct ConfirmedUser {
    name: String,
    confirmation_token: String,
}

async fn confirm_user(raw: User) -> Result<ConfirmedUser, BadInput> {
    tokio::task::yield_now().await;
    Ok(ConfirmedUser {
        name: raw.name.clone(),
        confirmation_token: format!("token-for-{}", raw.name),
    })
}

#[derive(TypestateFactory)]
#[factory(
    error = BadInput,
    finalize_async(via = confirm_user, into = ConfirmedUser, error = BadInput),
)]
struct User {
    #[field(required)]
    name: String,
}

#[tokio::test]
async fn standalone_async_finalize() {
    let bag = UserFactory::new().name("Carol".to_owned());

    // Sync finalize is still available.
    let raw = bag.finalize();
    assert_eq!(raw.name, "Carol");

    // Async finalize calls the hook.
    let confirmed = UserFactory::new()
        .name("Carol".to_owned())
        .finalize_async()
        .await
        .expect("hook ok");
    assert_eq!(
        confirmed,
        ConfirmedUser {
            name: "Carol".to_owned(),
            confirmation_token: "token-for-Carol".to_owned()
        }
    );
}

// ===========================================================================
// Async setter + Pipeline integration
// ===========================================================================

#[derive(Debug, Default)]
struct Hub;

#[derive(TypestateFactory)]
#[factory(error = BadInput, pipeline(carrier = Author))]
struct Order {
    #[field(required, setter = trim_sku_async, async_fn)]
    sku: String,
    #[field(required, setter = parse_quantity_async, async_fn, fallible)]
    quantity: u32,
}

async fn trim_sku_async(value: String) -> String {
    tokio::task::yield_now().await;
    value.trim().to_owned()
}

async fn parse_quantity_async(value: u32) -> Result<u32, BadInput> {
    tokio::task::yield_now().await;
    if value == 0 {
        Err(BadInput::Empty)
    } else {
        Ok(value)
    }
}

pipelined!(Author, ctx = Hub, error = BadInput);

impl<'a, S: 'a> Author<'a, S, Resolved> {
    fn state(&self) -> &S {
        self.0.state()
    }
}

#[tokio::test]
async fn pipeline_async_setter_chains_through_inflight() {
    let hub = Hub;
    let pipeline: Author<OrderFactory, Resolved> =
        Author(Pipeline::resolved(&hub, OrderFactory::new()));

    // The async setter `sku` opens an InFlight chain (Resolved → InFlight).
    // Subsequent setters chain through the pending future. Only one terminal
    // `.await?` is required to drive the entire chain.
    let bag: Author<OrderFactory<_, _>, Resolved> = pipeline
        .sku("  SKU-42  ".to_owned()) // Resolved → InFlight (async deferred)
        .quantity(5) // chains through InFlight
        .await
        .expect("chain ok");

    // Same shape works for the InFlight-input arms (it'd only differ if we
    // started with an InFlight pipeline). Confirm the values were trimmed +
    // validated.
    let raw = bag.0.into_state().finalize();
    assert_eq!(raw.sku, "SKU-42");
    assert_eq!(raw.quantity, 5);
}

#[tokio::test]
async fn pipeline_async_fallible_setter_propagates_error() {
    let hub = Hub;
    let pipeline: Author<OrderFactory, Resolved> =
        Author(Pipeline::resolved(&hub, OrderFactory::new()));

    let result = pipeline
        .sku("X".to_owned())
        .quantity(0) // fails inside the async transformer
        .await;

    match result {
        Err(BadInput::Empty) => {}
        Ok(_) => panic!("expected BadInput::Empty"),
    }
}

// ===========================================================================
// Q3: finalize used inside a `#[transitions]` body — most flexible path
// ===========================================================================
//
// The `finalize_async` hook is convenient when the work is self-contained.
// For finalizations that need extra context (e.g., a Hub reference) or
// produce different downstream phases conditionally, write a regular
// `#[transitions]` impl that calls `finalize()` (or `finalize_async()`)
// inside the body.

#[derive(Debug)]
struct Booked {
    sku: String,
    quantity: u32,
    receipt_id: u64,
}

// The transition is only callable on bags where every required field is Yes.
// Internally it calls `finalize()` and pulls a fresh receipt id from the Hub.
#[transitions(error = BadInput)]
impl<'a>
    Author<
        'a,
        OrderFactory<typestate_pipeline::Yes, typestate_pipeline::Yes>,
    >
{
    #[transition(into = Booked)]
    pub async fn book(
        state: OrderFactory<typestate_pipeline::Yes, typestate_pipeline::Yes>,
        ctx: &Hub,
    ) -> Result<Booked, BadInput> {
        let order = state.finalize();
        let _ = ctx;
        Ok(Booked {
            sku: order.sku,
            quantity: order.quantity,
            receipt_id: 1234,
        })
    }
}

#[tokio::test]
async fn transitions_body_calls_finalize() {
    let hub = Hub;
    let pipeline: Author<OrderFactory, Resolved> =
        Author(Pipeline::resolved(&hub, OrderFactory::new()));

    let booked = pipeline
        .sku("widget".to_owned())
        .quantity(3)
        .book() // uses finalize() inside the transition body
        .await
        .expect("book ok");

    assert_eq!(booked.state().sku, "widget");
    assert_eq!(booked.state().quantity, 3);
    assert_eq!(booked.state().receipt_id, 1234);
}
