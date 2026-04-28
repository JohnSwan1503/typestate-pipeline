//! Contract for `examples/transitions_sync_infallible.rs`.

use core::fmt;

use typestate_pipeline::{InFlight, Resolved, pipelined, transitions};

#[derive(Debug, Default)]
pub struct Hub;

#[derive(Debug)]
pub struct AppError;
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("err")
    }
}
impl std::error::Error for AppError {}

pipelined!(pub Author, ctx = Hub, error = AppError);

#[derive(Debug)]
pub struct Versioned;
#[derive(Debug)]
pub struct JobConfigured;

#[transitions]
impl<'a> Author<'a, Versioned> {
    #[transition(into = JobConfigured)]
    pub fn with_parallelism(state: Versioned, parallelism: u16) -> JobConfigured {
        let _ = (state, parallelism);
        JobConfigured
    }
}

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    fn check_resolved<'a>(
        _: fn(
            Author<'a, Versioned, Resolved>,
            u16,
        ) -> Author<'a, JobConfigured, Resolved>,
    ) {
    }
    check_resolved(<Author<'_, Versioned, Resolved>>::with_parallelism);

    fn check_inflight<'a>(
        _: fn(
            Author<'a, Versioned, InFlight>,
            u16,
        ) -> Author<'a, JobConfigured, InFlight>,
    ) {
    }
    check_inflight(<Author<'_, Versioned, InFlight>>::with_parallelism);
}

#[test]
fn surface_compiles() {
    surface_check();
}
