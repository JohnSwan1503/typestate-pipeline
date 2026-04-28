//! Async deferred transition: an `async fn` returning `Result<Next, E>`,
//! with no `deferred = false` on the `#[transition]` attribute (deferred
//! is the default for `async fn` bodies). Both arms produce
//! `InFlight` carriers — that's the lift that lets a chain like
//! `pipeline.tag_version(7).with_parallelism(8).deploy().await?` fold
//! every step into one terminal `.await?`.
//!
//! - **Resolved arm** wraps the body's future and lifts the carrier from
//!   `Resolved` to `InFlight`; returns `Author<'a, Next, InFlight>`.
//! - **InFlight arm** chains the body's future onto the pending future;
//!   returns `Author<'a, Next, InFlight>` again.
//!
//! For a transition where the next link needs the resolved value to
//! compute its arguments, pair this with `#[transition(deferred = false)]`
//! — see `./async_breakpoint.rs`.
//!
//! =============================================================================
//! Generated (sketch)
//! =============================================================================
//!
//!     impl<'a> Author<'a, Registered, Resolved> {
//!         pub fn tag_version(self, version: u32)
//!             -> Author<'a, Versioned, InFlight>;   // lift to InFlight
//!     }
//!     impl<'a> Author<'a, Registered, InFlight>
//!     where /* Send + 'a bounds */
//!     {
//!         pub fn tag_version(self, version: u32)
//!             -> Author<'a, Versioned, InFlight>;
//!     }

use std::fmt;

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
struct Registered {
    name: String,
}

#[derive(Debug)]
struct Versioned {
    name: String,
    version: u32,
}

#[derive(Debug)]
struct Deployed {
    name: String,
    version: u32,
}

#[transitions(error = AppError)]
impl<'a> Author<'a, Registered> {
    #[transition(into = Versioned)]
    pub async fn tag_version(
        state: Registered,
        ctx: &Hub,
        version: u32,
    ) -> Result<Versioned, AppError> {
        let _ = ctx;
        tokio::task::yield_now().await;
        Ok(Versioned {
            name: state.name,
            version,
        })
    }
}

#[transitions(error = AppError)]
impl<'a> Author<'a, Versioned> {
    #[transition(into = Deployed)]
    pub async fn deploy(state: Versioned, ctx: &Hub) -> Result<Deployed, AppError> {
        let _ = ctx;
        tokio::task::yield_now().await;
        Ok(Deployed {
            name: state.name,
            version: state.version,
        })
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let hub = Hub;
    let carrier: Author<Registered, Resolved> = Author(Pipeline::resolved(
        &hub,
        Registered {
            name: "ds".to_owned(),
        },
    ));

    // Two async transitions; one terminal `.await?` drives both.
    let deployed: Author<Deployed, Resolved> = carrier
        .tag_version(7)  // Resolved -> InFlight
        .deploy()        // InFlight  -> InFlight (folds onto the chain)
        .await           // InFlight  -> Resolved (drives the chain)
        .expect("chain ok");

    let state = deployed.0.into_state();
    assert_eq!(state.name, "ds");
    assert_eq!(state.version, 7);
}
