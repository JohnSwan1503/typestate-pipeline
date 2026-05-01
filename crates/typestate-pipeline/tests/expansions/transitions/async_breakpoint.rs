//! Contract for `examples/transitions_async_breakpoint.rs`.

use core::fmt;
use std::future::Future;

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
    #[transition(into = Versioned, breakpoint)]
    pub async fn confirm_and_tag(state: Registered, ctx: &Hub) -> Result<Versioned, AppError> {
        let _ = (state, ctx);
        Ok(Versioned)
    }
}

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    // Resolved arm: still async, returns Result<Resolved, E>.
    fn check_resolved<'a, Fut>(_: fn(Author<'a, Registered, Resolved>) -> Fut)
    where
        Fut: Future<Output = Result<Author<'a, Versioned, Resolved>, AppError>>,
    {
    }
    check_resolved(<Author<'_, Registered, Resolved>>::confirm_and_tag);

    // InFlight arm: also async, also a breakpoint — returns Result<Resolved, E>.
    fn check_inflight<'a, Fut>(_: fn(Author<'a, Registered, InFlight>) -> Fut)
    where
        Fut: Future<Output = Result<Author<'a, Versioned, Resolved>, AppError>>,
    {
    }
    check_inflight(<Author<'_, Registered, InFlight>>::confirm_and_tag);
}

#[test]
fn surface_compiles() {
    surface_check();
}
