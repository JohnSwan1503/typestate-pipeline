//! `impl_pipelined!(Author, ctx = Hub, error = AppError)` does everything
//! `pipelined!` does *except* declare the struct. Use it when you want
//! the struct to carry custom derives, extra generics, or different
//! field ordering — anything not covered by the conventional shape.
//!
//! Both macros also accept `tag = MyTag` as a final argument.
//!
//! =============================================================================
//! Generated (sketch)
//! =============================================================================
//!
//! Same as `../pipelined/minimal.rs` minus the struct declaration.

use std::fmt;

use typestate_pipeline::{Mode, Pipeline, Resolved, impl_pipelined};

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

// Hand-roll the struct. With this approach you can attach #[derive(Clone)]
// when `Pipeline: Clone` (e.g. resolved-mode + clonable state), pin extra
// generics like `<'a, K: Kind, S, M>`, etc. — none of which `pipelined!`'s
// fixed shape can express.
struct Author<'a, S, M = Resolved>(Pipeline<'a, Hub, (), S, AppError, M>)
where
    M: Mode<'a, S, AppError>;

// One line for all the trait plumbing.
impl_pipelined!(Author, ctx = Hub, error = AppError);

#[derive(Debug)]
#[allow(dead_code)]
struct Phase1;

fn main() {
    let hub = Hub;
    let carrier: Author<Phase1, Resolved> = Author(Pipeline::resolved(&hub, Phase1));
    let _ = carrier;
}
