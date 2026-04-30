use typestate_pipeline::{Pipeline, Resolved, TypestateFactory, pipelined};

use super::error::AppError;

#[derive(Debug, Default)]
pub struct Hub;

pipelined!(pub Author, ctx = Hub, error = AppError);

impl<'a, S: 'a> Author<'a, S, Resolved> {
    pub fn into_state(self) -> S {
        self.0.into_state()
    }
}

// ---------------------------------------------------------------------------
// Job — Pipeline-integrated bag with one `internal` field. Lives next
// to `Author` so the `pipeline(carrier = Author)` arm can reach the
// carrier's tuple-field internals.
// ---------------------------------------------------------------------------

#[derive(Debug, TypestateFactory)]
#[factory(pipeline(carrier = Author))]
pub struct Job {
    /// Phase-boundary plumbing — set positionally on `new(…)`.
    #[field(required, internal)]
    pub namespace: String,

    /// User-facing.
    #[field(required)]
    pub parallelism: u16,

    /// User-facing, optional with default.
    #[field(default = false)]
    pub verify: bool,
}

// ---------------------------------------------------------------------------
// Helper — wrap the (private-elsewhere) Author construction.
// ---------------------------------------------------------------------------

pub fn carrier<'a>(hub: &'a Hub, namespace: &str) -> Author<'a, JobFactory, Resolved> {
    Author(Pipeline::resolved(
        hub,
        JobFactory::new(namespace.to_owned()),
    ))
}
