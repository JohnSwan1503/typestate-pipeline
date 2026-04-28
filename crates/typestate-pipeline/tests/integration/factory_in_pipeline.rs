//! Demonstrates the integration story between `Pipeline` and `TypestateFactory`.
//!
//! The two patterns target different shapes of typestate. They compose: a
//! `Pipeline` phase can hold a `TypestateFactory` instantiation as its state,
//! and a transition out of the phase can call `.finalize()` on the bag to
//! produce the next-phase value. The Pipeline tracks *cross-phase* progress;
//! the bag tracks *within-phase* field accumulation.
//!
//! Layout below:
//!   Phase 1 (`Drafting`):  state = `UserDraftFactory<Y, Y, Y>` (a fully-set bag)
//!   Phase 2 (`Submitted`): state = `User`         (the finalized struct)
//!   Phase 3 (`Confirmed`): state = `Confirmation` (after async confirmation)
//!
//! Transitions:
//!   - `submit` (sync fallible): finalize the bag → `Submitted(User)`.
//!   - `confirm` (async deferred): hit the "server" → `Confirmed(Confirmation)`.

use std::{
    fmt,
    sync::atomic::{AtomicU64, Ordering},
};

use typestate_pipeline::{pipelined, transitions, Pipeline, Resolved, TypestateFactory};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
struct Server {
    next_confirmation_id: AtomicU64,
}

impl Server {
    fn confirm(&self, _user: &User) -> u64 {
        self.next_confirmation_id.fetch_add(1, Ordering::SeqCst) + 1
    }
}

#[derive(Debug)]
enum SubmitError {
    Empty(&'static str),
}

impl fmt::Display for SubmitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubmitError::Empty(field) => write!(f, "{field} is empty"),
        }
    }
}

impl std::error::Error for SubmitError {}

#[derive(Debug, Clone, TypestateFactory)]
struct User {
    #[field(required)]
    name: String,
    #[field(required)]
    email: String,
    #[field(default = 18)]
    age: u32,
}

// Carried-state types for each phase. Phase 1 holds a fully-populated bag;
// phases 2 and 3 hold the finalized data.
struct Drafting(UserFactory<typestate_pipeline::Yes, typestate_pipeline::Yes, typestate_pipeline::Yes>);
struct Submitted(User);
struct Confirmed {
    user: User,
    confirmation_id: u64,
}

// ---------------------------------------------------------------------------
// Pipeline carrier
// ---------------------------------------------------------------------------

pipelined!(Author, ctx = Server, error = SubmitError);

impl<'a, S: 'a> Author<'a, S, Resolved> {
    fn state(&self) -> &S {
        self.0.state()
    }
}

// ---------------------------------------------------------------------------
// Drafting → Submitted: sync fallible — finalize the bag, validate.
// Submitted → Confirmed: async deferred — round-trip to the "server".
// ---------------------------------------------------------------------------

#[transitions(error = SubmitError)]
impl<'a> Author<'a, Drafting> {
    /// Finalize the bag, validate the resulting `User`, and advance.
    #[transition(into = Submitted)]
    pub fn submit(state: Drafting) -> Result<Submitted, SubmitError> {
        let user = state.0.finalize();
        if user.name.is_empty() {
            return Err(SubmitError::Empty("name"));
        }
        if user.email.is_empty() {
            return Err(SubmitError::Empty("email"));
        }
        Ok(Submitted(user))
    }
}

#[transitions(error = SubmitError)]
impl<'a> Author<'a, Submitted> {
    #[transition(into = Confirmed)]
    pub async fn confirm(state: Submitted, ctx: &Server) -> Result<Confirmed, SubmitError> {
        let id = ctx.confirm(&state.0);
        Ok(Confirmed {
            user: state.0,
            confirmation_id: id,
        })
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

fn drafting<'a>(server: &'a Server, user: User) -> Author<'a, Drafting> {
    // Build a fully-set bag from a User. In real code this would happen via
    // the bag's setters interactively; here we round-trip for compactness.
    let bag = UserFactory::new()
        .name(user.name)
        .email(user.email)
        .with_age(user.age);
    Author(Pipeline::resolved(server, Drafting(bag)))
}

#[tokio::test]
async fn full_chain_bag_into_pipeline() {
    let server = Server::default();
    let pipeline = drafting(
        &server,
        User {
            name: "Alice".to_owned(),
            email: "alice@example.com".to_owned(),
            age: 30,
        },
    );

    // Sync fallible (submit) folds its Result into the InFlight chain that
    // confirm() opens — single terminal `.await?`.
    let confirmed: Author<Confirmed, Resolved> = pipeline
        .submit() // Result<Author<Submitted, Resolved>, SubmitError>
        .expect("submit should succeed")
        .confirm() // Author<Confirmed, InFlight>
        .await
        .expect("confirm should succeed");

    assert_eq!(confirmed.state().user.name, "Alice");
    assert_eq!(confirmed.state().confirmation_id, 1);
}

#[tokio::test]
async fn validation_failure_at_bag_finalize() {
    let server = Server::default();
    let pipeline = drafting(
        &server,
        User {
            name: String::new(),
            email: "no-name@example.com".to_owned(),
            age: 0,
        },
    );

    match pipeline.submit() {
        Err(SubmitError::Empty(field)) => assert_eq!(field, "name"),
        Ok(_) => panic!("expected validation error"),
    }
}
