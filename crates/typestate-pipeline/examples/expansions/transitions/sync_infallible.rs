//! Sync infallible transition: a `fn` returning a non-`Result`. The
//! macro emits two arms:
//!
//! - **Resolved arm** — applies the body inline; returns the carrier in
//!   `Resolved` mode for the next state.
//! - **InFlight arm** — awaits the pending state, applies the body, and
//!   re-wraps the result as `InFlight` so the chain keeps folding into a
//!   single terminal `.await?`.
//!
//! =============================================================================
//! Generated (sketch)
//! =============================================================================
//!
//!     impl<'a> Author<'a, Versioned, Resolved> {
//!         pub fn with_parallelism(self, parallelism: u16)
//!             -> Author<'a, JobConfigured, Resolved>;
//!     }
//!     impl<'a> Author<'a, Versioned, InFlight>
//!     where
//!         Versioned: Send + 'a,
//!         JobConfigured: Send + 'a,
//!     {
//!         pub fn with_parallelism(self, parallelism: u16)
//!             -> Author<'a, JobConfigured, InFlight>;
//!     }

use core::fmt;

use typestate_pipeline::{Pipeline, Resolved, pipelined, transitions};

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
#[allow(dead_code)]
struct Versioned {
    name: String,
    version: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct JobConfigured {
    name: String,
    version: u32,
    parallelism: u16,
}

#[transitions]
impl<'a> Author<'a, Versioned> {
    #[transition(into = JobConfigured)]
    pub fn with_parallelism(state: Versioned, parallelism: u16) -> JobConfigured {
        JobConfigured {
            name: state.name,
            version: state.version,
            parallelism,
        }
    }
}

fn main() {
    let hub = Hub;
    let carrier: Author<Versioned, Resolved> = Author(Pipeline::resolved(
        &hub,
        Versioned {
            name: "ds".to_owned(),
            version: 1,
        },
    ));

    let configured: Author<JobConfigured, Resolved> = carrier.with_parallelism(8);
    let job = configured.0.into_state();
    assert_eq!(job.parallelism, 8);
    assert_eq!(job.version, 1);
}
