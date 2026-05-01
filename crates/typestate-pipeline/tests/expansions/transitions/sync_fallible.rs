//! Contract for `examples/transitions_sync_fallible.rs`.

use core::fmt;

use typestate_pipeline::{InFlight, Resolved, pipelined, transitions};

#[derive(Debug, Default)]
pub struct Hub;

#[derive(Debug)]
pub struct AppError(&'static str);
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
impl std::error::Error for AppError {}

pipelined!(pub Author, ctx = Hub, error = AppError);

#[derive(Debug)]
pub struct JobConfigured;

#[transitions(error = AppError)]
impl<'a> Author<'a, JobConfigured> {
    #[transition(into = JobConfigured)]
    pub fn validate(state: JobConfigured) -> Result<JobConfigured, AppError> {
        Ok(state)
    }
}

// `validate` body is the no-op; ensure the `state` binding doesn't trigger
// unused-variable warnings by forcing it through `Ok(state)`.

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    // Resolved arm returns `Result<NextCarrier, E>`.
    fn check_resolved<'a>(
        _: fn(
            Author<'a, JobConfigured, Resolved>,
        ) -> Result<Author<'a, JobConfigured, Resolved>, AppError>,
    ) {
    }
    check_resolved(<Author<'_, JobConfigured, Resolved>>::validate);

    // InFlight arm folds the Result into the pending — no Result at the
    // call site, returns InFlight directly.
    fn check_inflight<'a>(
        _: fn(Author<'a, JobConfigured, InFlight>) -> Author<'a, JobConfigured, InFlight>,
    ) {
    }
    check_inflight(<Author<'_, JobConfigured, InFlight>>::validate);
}

#[test]
fn surface_compiles() {
    surface_check();
}
