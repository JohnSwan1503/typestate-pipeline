use typestate_pipeline::{
    BoxFuture, InFlight, Pipeline, Resolved, TypestateFactory, pipelined, transitions,
};

use super::error::AppError;
use super::phases::{Drafted, Published, Versioned};

#[derive(Debug)]
pub struct Hub;

// Single line declares the carrier struct + Pipelined impl + IntoFuture
// forwarding.
pipelined!(pub Author, ctx = Hub, error = AppError);

impl<'a, S: 'a> Author<'a, S, Resolved> {
    pub fn into_state(self) -> S {
        self.0.into_state()
    }
}

// `error =` omitted on the `#[transitions]` impl — read from
// `<Self as Pipelined<'a>>::Error`.
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

// ---------------------------------------------------------------------------
// Profile — Pipeline-integrated bag without an `error =` arg. Lives
// alongside `Author` so the derive's `pipeline(carrier = Author)` arm
// has direct access to the carrier's internals. No fallible setters →
// no error needed at the bag site; the pipeline arms infer the error
// type from the carrier.
// ---------------------------------------------------------------------------

#[derive(TypestateFactory)]
#[factory(pipeline(carrier = Author))]
pub struct Profile {
    #[field(required)]
    pub handle: String,
    #[field(default = 0)]
    pub age: u32,
}

// ---------------------------------------------------------------------------
// Test helpers — wrap the (private-elsewhere) Author construction.
// ---------------------------------------------------------------------------

pub fn drafted<'a>(hub: &'a Hub, name: &str) -> Author<'a, Drafted, Resolved> {
    Author(Pipeline::resolved(
        hub,
        Drafted {
            name: name.to_owned(),
        },
    ))
}

pub fn empty_profile<'a>(hub: &'a Hub) -> Author<'a, ProfileFactory, Resolved> {
    Author(Pipeline::resolved(hub, ProfileFactory::new()))
}

pub fn drafted_inflight<'a>(
    hub: &'a Hub,
    pending: BoxFuture<'a, Result<Drafted, AppError>>,
) -> Author<'a, Drafted, InFlight> {
    Author(Pipeline::in_flight(hub, pending))
}
