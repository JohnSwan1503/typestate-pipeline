#![allow(unused)]

#[path = "async/tests/shared.rs"]
mod shared;

use shared::*;

#[path = "async/tests/standalone_async_setter_non_fallible.rs"]
pub mod standalone_async_setter_non_fallible;

#[path = "async/tests/standalone_async_setter_fallible_failure.rs"]
pub mod standalone_async_setter_fallible_failure;

#[path = "async/tests/standalone_async_finalize.rs"]
pub mod standalone_async_finalize;

#[path = "async/tests/pipeline_async_setter_chains_through_inflight.rs"]
pub mod pipeline_async_setter_chains_through_inflight;

#[path = "async/tests/pipeline_async_fallible_setter_propagates_error.rs"]
pub mod pipeline_async_fallible_setter_propagates_error;

#[path = "async/tests/transitions_body_calls_finalize.rs"]
pub mod transitions_body_calls_finalize;

// ---------------------------------------------------------------------------
// Async setter (async_fn) returns the next bag after `.await`.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn standalone_async_setter_non_fallible() {
    standalone_async_setter_non_fallible::main().await;
}

// ---------------------------------------------------------------------------
// Async fallible setter's `Err` after `.await` propagates.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn standalone_async_setter_fallible_failure() {
    standalone_async_setter_fallible_failure::main().await;
}

// ---------------------------------------------------------------------------
// `finalize_async` routes through the user-supplied async hook.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn standalone_async_finalize() {
    standalone_async_finalize::main().await;
}

// ---------------------------------------------------------------------------
// Pipeline-arm async setter opens an InFlight chain.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn pipeline_async_setter_chains_through_inflight() {
    pipeline_async_setter_chains_through_inflight::main().await;
}

// ---------------------------------------------------------------------------
// Pipeline-arm async fallible setter surfaces error at terminal `.await?`.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn pipeline_async_fallible_setter_propagates_error() {
    pipeline_async_fallible_setter_propagates_error::main().await;
}

// ---------------------------------------------------------------------------
// `#[transitions]` body can call `finalize()` mid-chain.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn transitions_body_calls_finalize() {
    transitions_body_calls_finalize::main().await;
}
