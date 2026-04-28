//! Contract for `examples/inspect_combinator.rs`.

use std::fmt;

use typestate_pipeline::{InFlight, Resolved, pipelined};

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
pub struct Phase1;

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    // Defining a generic fn whose body calls `inspect` with the
    // documented closure signature is enough — Rust type-checks the body
    // at definition time, so a missing or wrongly-shaped `inspect` is
    // a compile error here.

    // Resolved-mode inspect: F: FnOnce(&Self), returns Self.
    fn _check_resolved<'a>(carrier: Author<'a, Phase1, Resolved>) -> Author<'a, Phase1, Resolved> {
        carrier.inspect(|_c: &Author<'a, Phase1, Resolved>| {})
    }

    // InFlight-mode inspect: F: FnOnce(&Author<'a, S, Resolved>) + Send + 'a,
    // returns Self (still InFlight).
    fn _check_inflight<'a>(carrier: Author<'a, Phase1, InFlight>) -> Author<'a, Phase1, InFlight>
    where
        Phase1: Send + 'a,
    {
        carrier.inspect(|_c: &Author<'a, Phase1, Resolved>| {})
    }
}

#[test]
fn surface_compiles() {
    surface_check();
}
