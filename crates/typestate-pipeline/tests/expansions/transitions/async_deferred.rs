//! Contract for `examples/transitions_async_deferred.rs`.

use std::fmt;

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
pub struct Registered;
#[derive(Debug)]
pub struct Versioned;

#[transitions(error = AppError)]
impl<'a> Author<'a, Registered> {
    #[transition(into = Versioned)]
    pub async fn tag_version(
        state: Registered,
        ctx: &Hub,
        version: u32,
    ) -> Result<Versioned, AppError> {
        let _ = (state, ctx, version);
        Ok(Versioned)
    }
}

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    // Resolved arm: lifts to InFlight (deferred = true is the default).
    fn check_resolved<'a>(
        _: fn(
            Author<'a, Registered, Resolved>,
            u32,
        ) -> Author<'a, Versioned, InFlight>,
    ) {
    }
    check_resolved(<Author<'_, Registered, Resolved>>::tag_version);

    // InFlight arm: stays InFlight.
    fn check_inflight<'a>(
        _: fn(
            Author<'a, Registered, InFlight>,
            u32,
        ) -> Author<'a, Versioned, InFlight>,
    ) {
    }
    check_inflight(<Author<'_, Registered, InFlight>>::tag_version);
}

#[test]
fn surface_compiles() {
    surface_check();
}
