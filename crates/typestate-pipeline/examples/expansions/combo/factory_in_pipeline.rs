//! Combination case: a standalone `TypestateFactory` bag built outside
//! the carrier, then handed to a `#[transitions]` body that consumes it
//! mid-chain. The bag's `finalize()` produces a regular struct, which the
//! transition uses to compute the next state.
//!
//! This is the canonical bridge between "named-field accumulator" and
//! "phase machine" when the bag is built independently — e.g. handed in
//! from configuration. For the alternative — emit setters directly on
//! the carrier — see `../factory/pipeline_arm.rs`.

use std::fmt;

use typestate_pipeline::{Pipeline, Resolved, TypestateFactory, pipelined, transitions};

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

#[derive(Debug, TypestateFactory)]
#[allow(dead_code)]
struct Settings {
    #[field(required)]
    parallelism: u16,
    #[field(default = false)]
    verify: bool,
}

pipelined!(Author, ctx = Hub, error = AppError);

#[derive(Debug)]
#[allow(dead_code)]
struct Configured {
    settings: Settings,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Deployed {
    settings: Settings,
}

#[transitions(error = AppError)]
impl<'a> Author<'a, Configured> {
    /// Lifts `Configured` to `Deployed`. The `Settings` it carries was
    /// built outside the carrier (see `main` below).
    #[transition(into = Deployed)]
    pub fn deploy(state: Configured, ctx: &'a Hub) -> Deployed {
        let _ = ctx;
        Deployed {
            settings: state.settings,
        }
    }
}

fn main() {
    let hub = Hub;

    // Build the bag wholly outside the carrier. The derive emits
    // `SettingsFactory` — finalize hands back a real `Settings`.
    let settings = SettingsFactory::new().parallelism(8).with_verify(true).finalize();

    // Walk into the carrier with the assembled phase state.
    let carrier: Author<Configured, Resolved> =
        Author(Pipeline::resolved(&hub, Configured { settings }));

    let deployed = carrier.deploy();
    let state = deployed.0.into_state();
    assert_eq!(state.settings.parallelism, 8);
    assert!(state.settings.verify);
}
