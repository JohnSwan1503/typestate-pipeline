#![allow(unused)]

#[path = "factory_in_pipeline/tests/shared.rs"]
mod shared;

use shared::*;

#[path = "factory_in_pipeline/tests/full_chain_bag_into_pipeline.rs"]
pub mod full_chain_bag_into_pipeline;

#[path = "factory_in_pipeline/tests/validation_failure_at_bag_finalize.rs"]
pub mod validation_failure_at_bag_finalize;

// ---------------------------------------------------------------------------
// Bag finalization folds into a pipeline chain — sync-fallible Resolved arm
// hands back a `Result` synchronously; the async-deferred InFlight arm folds
// onto the same chain so the whole `submit() -> confirm() -> .await` flow
// resolves at one terminal `.await?`.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn full_chain_bag_into_pipeline() {
    full_chain_bag_into_pipeline::main().await;
}

// ---------------------------------------------------------------------------
// Bag-finalize errors short-circuit the pipeline chain — validation failure
// inside the sync-fallible body returns `Err` at the call site, no `confirm`
// future is constructed.
// ---------------------------------------------------------------------------

#[test]
fn validation_failure_at_bag_finalize() {
    validation_failure_at_bag_finalize::main();
}
