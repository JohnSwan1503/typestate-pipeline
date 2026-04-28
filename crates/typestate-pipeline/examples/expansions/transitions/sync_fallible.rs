//! Sync fallible transition: a `fn` returning `Result<Next, E>`.
//!
//! - **Resolved arm** returns `Result<Author<Next, Resolved>, E>` — the
//!   error is right at the call site, no `.await` involved.
//! - **InFlight arm** *folds* the body's `Result` into the pending future;
//!   no `Result` at the call site, the error surfaces at the chain's
//!   terminal `.await?`.
//!
//! =============================================================================
//! Generated (sketch)
//! =============================================================================
//!
//!     impl<'a> Author<'a, JobConfigured, Resolved> {
//!         pub fn validate(self)
//!             -> Result<Author<'a, JobConfigured, Resolved>, AppError>;
//!     }
//!     impl<'a> Author<'a, JobConfigured, InFlight>
//!     where /* Send + 'a bounds */
//!     {
//!         pub fn validate(self) -> Author<'a, JobConfigured, InFlight>;
//!         //                       ^^^ Result is folded into the pending future
//!     }

use core::fmt;

use typestate_pipeline::{Pipeline, Resolved, pipelined, transitions};

#[derive(Debug, Default)]
struct Hub;

#[derive(Debug)]
struct AppError(&'static str);
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
impl std::error::Error for AppError {}

pipelined!(Author, ctx = Hub, error = AppError);

#[derive(Debug)]
struct JobConfigured {
    parallelism: u16,
    verified: bool,
}

#[transitions(error = AppError)]
impl<'a> Author<'a, JobConfigured> {
    #[transition(into = JobConfigured)]
    pub fn validate(mut state: JobConfigured) -> Result<JobConfigured, AppError> {
        if state.parallelism == 0 {
            return Err(AppError("parallelism must be > 0"));
        }
        state.verified = true;
        Ok(state)
    }
}

fn main() -> Result<(), AppError> {
    let hub = Hub;
    let carrier: Author<JobConfigured, Resolved> = Author(Pipeline::resolved(
        &hub,
        JobConfigured {
            parallelism: 4,
            verified: false,
        },
    ));

    // Resolved arm hands back a Result directly — no .await needed.
    let validated: Author<JobConfigured, Resolved> = carrier.validate()?;
    assert!(validated.0.into_state().verified);
    Ok(())
}
