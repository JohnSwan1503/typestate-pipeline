#![allow(unused)]

#[path = "core/tests/shared.rs"]
mod shared;

use shared::*;

#[path = "core/tests/full_chain_with_resolved_breakpoint_in_middle.rs"]
pub mod full_chain_with_resolved_breakpoint_in_middle;

#[path = "core/tests/breakpoint_forces_explicit_await.rs"]
pub mod breakpoint_forces_explicit_await;

#[path = "core/tests/sync_fallible_resolved_returns_result_directly.rs"]
pub mod sync_fallible_resolved_returns_result_directly;

#[path = "core/tests/sync_fallible_propagates_through_inflight_chain.rs"]
pub mod sync_fallible_propagates_through_inflight_chain;

#[path = "core/tests/intofuture_resolves_inflight_back_to_resolved.rs"]
pub mod intofuture_resolves_inflight_back_to_resolved;

// ---------------------------------------------------------------------------
// Full chain folds every body shape into one terminal `.await?`.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn full_chain_with_resolved_breakpoint_in_middle() {
    full_chain_with_resolved_breakpoint_in_middle::main().await;
}

// ---------------------------------------------------------------------------
// Async breakpoint forces an explicit `.await` mid-chain.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn breakpoint_forces_explicit_await() {
    breakpoint_forces_explicit_await::main().await;
}

// ---------------------------------------------------------------------------
// Sync fallible Resolved arm hands back `Result` directly.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn sync_fallible_resolved_returns_result_directly() {
    sync_fallible_resolved_returns_result_directly::main().await;
}

// ---------------------------------------------------------------------------
// Sync fallible inside InFlight surfaces error at terminal `.await?`.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn sync_fallible_propagates_through_inflight_chain() {
    sync_fallible_propagates_through_inflight_chain::main().await;
}

// ---------------------------------------------------------------------------
// IntoFuture drives an InFlight carrier to its Resolved successor.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn intofuture_resolves_inflight_back_to_resolved() {
    intofuture_resolves_inflight_back_to_resolved::main().await;
}
