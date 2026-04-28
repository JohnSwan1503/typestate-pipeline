//! `pipelined!(pub Author, ctx = Hub, error = AppError)` is the
//! one-liner that declares a typestate carrier and all the trait
//! plumbing the `#[transitions]` and `#[derive(TypestateFactory)]`
//! macros need to introspect it. It emits:
//!
//! 1. The newtype struct `pub struct Author<'a, S, M = Resolved>(...)`.
//! 2. The [`Pipelined<'a>`] impl, projecting `Ctx` / `Error` / `Tag` and
//!    the `Resolved<NS>` / `InFlight<NS>` GAT successors.
//! 3. An [`IntoFuture`] impl on `Author<'a, S, InFlight>` so awaiting an
//!    in-flight carrier yields a `Result<Author<'a, S, Resolved>, Error>`.
//! 4. A chainable `inspect(|c| …)` combinator on both modes.
//!
//! Use `impl_pipelined!` (see `../impl_pipelined/minimal.rs`) when you
//! need to hand-roll the struct (custom derives, extra generics, …). Use
//! `pipelined!(…, tag = MyTag)` (see `./with_tag.rs`) to attach a
//! phantom tag.
//!
//! =============================================================================
//! Generated (sketch)
//! =============================================================================
//!
//!     pub struct Author<'a, S, M = Resolved>(
//!         Pipeline<'a, Hub, (), S, AppError, M>,
//!     )
//!     where M: Mode<'a, S, AppError>;
//!
//!     impl<'a, S: 'a, M> Pipelined<'a> for Author<'a, S, M>
//!     where M: Mode<'a, S, AppError>,
//!     {
//!         type Ctx = Hub;
//!         type Error = AppError;
//!         type Tag = ();
//!         type State = S;
//!         type Mode = M;
//!         type Resolved<NS: 'a> = Author<'a, NS, Resolved>;
//!         type InFlight<NS: Send + 'a> = Author<'a, NS, InFlight>;
//!     }
//!
//!     impl<'a, S> IntoFuture for Author<'a, S, InFlight>
//!     where S: Send + 'a, AppError: Send + 'a, Hub: Sync + 'a,
//!     {
//!         type Output = Result<Author<'a, S, Resolved>, AppError>;
//!         type IntoFuture = BoxFuture<'a, Self::Output>;
//!         fn into_future(self) -> Self::IntoFuture { ... }
//!     }
//!
//!     // `inspect` is documented in `./inspect_combinator.rs`.
//!
//! [`Pipelined<'a>`]: typestate_pipeline::Pipelined
//! [`IntoFuture`]: core::future::IntoFuture

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

// One line; no struct definition needed. (Drop the leading `pub` to match
// the visibility of the surrounding types — adding `pub` works too, but
// it requires the ctx/error/tag types to also be `pub` since the
// generated `Pipelined` impl exposes them.)
pipelined!(Author, ctx = Hub, error = AppError);

#[derive(Debug)]
#[allow(dead_code)]
struct Phase1;

fn main() {
    let hub = Hub;
    // Construct via the inner `Pipeline::resolved` and the carrier newtype.
    let carrier: Author<Phase1, Resolved> = Author(Pipeline::resolved(&hub, Phase1));
    // The carrier is a real value with all the trait plumbing in place;
    // `#[transitions]` impls on it just work.
    let _ = carrier;
}
