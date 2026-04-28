//! Async **breakpoint** transition: an `async fn` body marked with
//! `#[transition(into = Next, deferred = false)]`. Both arms keep the
//! method `async` and resolve to `Resolved` mode — the chain *breaks*
//! here, so the caller must `.await?` (or otherwise drive the future)
//! before the next link.
//!
//! Useful when the next transition's argument value needs the resolved
//! state — e.g. computing a version bump from the current version, or
//! matching on a phase to decide which downstream transition to call.
//!
//! =============================================================================
//! Generated (sketch)
//! =============================================================================
//!
//!     impl<'a> Author<'a, Registered, Resolved> {
//!         pub async fn confirm_and_tag(self)
//!             -> Result<Author<'a, Versioned, Resolved>, AppError>;
//!         //                                  ^^^^^^^^   resolved, not InFlight
//!     }
//!     impl<'a> Author<'a, Registered, InFlight>
//!     where /* Send + 'a bounds */
//!     {
//!         pub async fn confirm_and_tag(self)
//!             -> Result<Author<'a, Versioned, Resolved>, AppError>;
//!         //                                  ^^^^^^^^   also a breakpoint
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
struct Registered {
    name: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Versioned {
    name: String,
    version: u32,
}

#[transitions(error = AppError)]
impl<'a> Author<'a, Registered> {
    /// Breakpoint — caller must await this before the next link.
    #[transition(into = Versioned, deferred = false)]
    pub async fn confirm_and_tag(
        state: Registered,
        ctx: &Hub,
    ) -> Result<Versioned, AppError> {
        let _ = ctx;
        tokio::task::yield_now().await;
        Ok(Versioned {
            name: state.name,
            version: 1,
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

    // Breakpoint: the chain pauses at the .await — the next link sees a
    // Resolved carrier whose state is fully observable.
    let versioned: Author<Versioned, Resolved> =
        carrier.confirm_and_tag().await.expect("confirm");

    assert_eq!(versioned.0.into_state().version, 1);
}
