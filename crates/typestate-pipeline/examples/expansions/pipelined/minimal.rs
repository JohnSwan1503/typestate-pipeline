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
