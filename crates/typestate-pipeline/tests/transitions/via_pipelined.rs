//! End-to-end exercise of the [`Pipelined`] trait integration: when a carrier
//! implements `Pipelined<'a>` (via [`pipelined!`] or [`impl_pipelined!`]),
//! `#[transitions]` and `#[factory(pipeline(...))]` can both omit `error = …`
//! — the error type is read from the carrier's `Pipelined::Error` projection,
//! and destination types come from the GAT projections.

use std::{fmt, future::IntoFuture};

use typestate_pipeline::{pipelined, transitions, Pipeline, Resolved, TypestateFactory};

#[derive(Debug)]
struct Hub;

#[derive(Debug)]
enum AppError {
    Bad(&'static str),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Bad(m) => write!(f, "bad: {m}"),
        }
    }
}
impl std::error::Error for AppError {}

// Single line declares the carrier struct + Pipelined impl + IntoFuture
// forwarding.
pipelined!(Author, ctx = Hub, error = AppError);

#[derive(Debug, Clone)]
struct Drafted {
    name: String,
}

#[derive(Debug, Clone)]
struct Versioned {
    name: String,
    version: u32,
}

#[derive(Debug, Clone)]
struct Published {
    name: String,
    version: u32,
}

// `error =` omitted — read from `<Self as Pipelined<'a>>::Error`.
#[transitions]
impl<'a> Author<'a, Drafted> {
    #[transition(into = Versioned)]
    pub async fn tag(state: Drafted, ctx: &Hub, version: u32) -> Result<Versioned, AppError> {
        let _ = ctx;
        if version == 0 {
            return Err(AppError::Bad("version must be > 0"));
        }
        Ok(Versioned {
            name: state.name,
            version,
        })
    }
}

#[transitions]
impl<'a> Author<'a, Versioned> {
    #[transition(into = Published)]
    pub fn publish(state: Versioned) -> Result<Published, AppError> {
        if state.name.is_empty() {
            return Err(AppError::Bad("name"));
        }
        Ok(Published {
            name: state.name,
            version: state.version,
        })
    }
}

#[tokio::test]
async fn transitions_chain_without_error_arg() {
    let hub = Hub;
    let initial = Author(Pipeline::resolved(
        &hub,
        Drafted {
            name: "alpha".to_owned(),
        },
    ));

    // Resolved → InFlight (deferred async) → InFlight (sync fallible folds in)
    // → terminal `.await?`.
    let published = initial
        .tag(7)
        .publish()
        .await
        .expect("chain should succeed");

    let state = published.0.into_state();
    assert_eq!(state.name, "alpha");
    assert_eq!(state.version, 7);
}

#[tokio::test]
async fn transitions_chain_propagates_error() {
    let hub = Hub;
    let initial = Author(Pipeline::resolved(
        &hub,
        Drafted {
            name: "alpha".to_owned(),
        },
    ));

    // tag(0) fails — error must bubble through the chain.
    let result = initial.tag(0).publish().await;
    match result {
        Err(AppError::Bad(m)) => assert_eq!(m, "version must be > 0"),
        Ok(_) => panic!("expected error"),
    }
}

// ---------------------------------------------------------------------------
// TypestateFactory pipeline integration without `error =`.
// ---------------------------------------------------------------------------

// No fallible setters → no error needed at the bag site. Pipeline arms infer
// the error type from the carrier.
#[derive(TypestateFactory)]
#[factory(pipeline(carrier = Author))]
struct Profile {
    #[field(required)]
    handle: String,
    #[field(default = 0)]
    age: u32,
}

#[tokio::test]
async fn factory_pipeline_arms_without_error_arg() {
    // Round-trip through the carrier: open a Resolved Author with an empty
    // bag, drive setters via the generated pipeline arms, finalize.
    let hub = Hub;
    let initial: Author<ProfileFactory, Resolved> =
        Author(Pipeline::resolved(&hub, ProfileFactory::new()));

    let bag_carrier = initial
        .handle("alice".to_owned()) // pipeline-integrated setter
        .age_default(); // pipeline-integrated default helper

    let profile = bag_carrier.0.into_state().finalize();
    assert_eq!(profile.handle, "alice");
    assert_eq!(profile.age, 0);
}

// IntoFuture is supplied by pipelined!; await on InFlight resolves to Resolved.
#[tokio::test]
async fn intofuture_provided_by_pipelined() {
    let hub = Hub;
    let pending: typestate_pipeline::BoxFuture<'_, Result<Drafted, AppError>> =
        Box::pin(async {
            Ok(Drafted {
                name: "x".to_owned(),
            })
        });
    let in_flight = Author(Pipeline::in_flight(&hub, pending));
    let resolved: Author<Drafted, Resolved> = in_flight.into_future().await.unwrap();
    assert_eq!(resolved.0.into_state().name, "x");
}
