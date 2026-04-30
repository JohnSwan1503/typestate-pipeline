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
