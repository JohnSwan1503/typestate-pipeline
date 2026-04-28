//! `pipelined!(…, tag = MyTag)` (or `impl_pipelined!(…, tag = MyTag)`)
//! attaches a phantom tag to the carrier. The tag is invariant in the
//! type system but doesn't carry runtime data — useful when you want to
//! statically distinguish "two pipelines that look the same" (e.g. one
//! per dataset kind) without changing the value-level shape.
//!
//! When omitted, `tag` defaults to `()`.
//!
//! =============================================================================
//! Generated (sketch) — diff from `./minimal.rs`
//! =============================================================================
//!
//!     pub struct Author<'a, S, M = Resolved>(
//!         Pipeline<'a, Hub, RawKind, S, AppError, M>,
//!     )                          //  ^^^^^^^ tag slot
//!     where M: Mode<'a, S, AppError>;
//!
//!     impl<…> Pipelined<'a> for Author<'a, S, M> {
//!         type Tag = RawKind;        // <-- the only Pipelined associated-type diff
//!         …
//!     }

use std::fmt;

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

/// Phantom tag — never instantiated, used at the type level only.
#[derive(Debug)]
enum RawKind {}

pipelined!(Author, ctx = Hub, error = AppError, tag = RawKind);

#[derive(Debug)]
#[allow(dead_code)]
struct Phase1;

fn main() {
    let hub = Hub;
    let carrier: Author<Phase1, Resolved> = Author(Pipeline::resolved(&hub, Phase1));
    let _ = carrier;
}
