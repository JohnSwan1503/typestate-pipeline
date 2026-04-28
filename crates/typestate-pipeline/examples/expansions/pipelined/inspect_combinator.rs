//! `pipelined!` / `impl_pipelined!` both emit a chainable `inspect(|c| …)`
//! combinator on Resolved *and* InFlight modes. It runs a closure against
//! the carrier (or, for InFlight, the eventual resolved carrier) without
//! changing the typestate, so the chain continues unchanged.
//!
//! - **Resolved arm**: `inspect(F: FnOnce(&Self))` — closure runs
//!   synchronously against the resolved carrier; the same carrier is
//!   returned.
//! - **InFlight arm**: `inspect(F: FnOnce(&Author<'a, S, Resolved>) + Send + 'a)`
//!   — the closure runs after the pending future resolves, against a
//!   temporary `Resolved` view; the carrier is rewrapped as `InFlight`
//!   so subsequent transitions keep folding.
//!
//! =============================================================================
//! Generated (sketch)
//! =============================================================================
//!
//!     impl<'a, S: 'a> Author<'a, S, Resolved> {
//!         pub fn inspect<F>(self, f: F) -> Self
//!         where F: FnOnce(&Self);
//!     }
//!     impl<'a, S> Author<'a, S, InFlight>
//!     where S: Send + 'a, AppError: Send + 'a, Hub: Sync + 'a,
//!     {
//!         pub fn inspect<F>(self, f: F) -> Self
//!         where F: FnOnce(&Author<'a, S, Resolved>) + Send + 'a;
//!     }

use core::fmt;

use typestate_pipeline::{Pipeline, Resolved, pipelined};

#[derive(Debug, Default)]
struct Hub;

#[derive(Debug)]
struct AppError;
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("err")
    }
}
impl std::error::Error for AppError {}

pipelined!(Author, ctx = Hub, error = AppError);

#[derive(Debug)]
struct Phase1 {
    label: String,
}

fn main() {
    let hub = Hub;
    let carrier: Author<Phase1, Resolved> = Author(Pipeline::resolved(
        &hub,
        Phase1 {
            label: "first".to_owned(),
        },
    ));

    // Resolved-side inspect: closure observes the carrier; carrier is
    // returned unchanged so the chain continues.
    let carrier = carrier.inspect(|c| {
        println!("inspecting {}", c.0.state().label);
    });

    let _ = carrier.0.into_state();
}
