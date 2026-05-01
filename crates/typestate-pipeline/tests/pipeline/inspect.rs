#![allow(unused)]

#[path = "inspect/tests/shared.rs"]
mod shared;

pub(self) use shared::*;

#[path = "inspect/tests/resolved_inspect_runs_sync_and_preserves_chain.rs"]
pub mod resolved_inspect_runs_sync_and_preserves_chain;

#[path = "inspect/tests/resolved_inspect_does_not_change_typestate.rs"]
pub mod resolved_inspect_does_not_change_typestate;

#[path = "inspect/tests/inflight_inspect_runs_after_pending_resolves.rs"]
pub mod inflight_inspect_runs_after_pending_resolves;

#[path = "inspect/tests/inflight_inspect_chains_through_subsequent_transitions.rs"]
pub mod inflight_inspect_chains_through_subsequent_transitions;

// ---------------------------------------------------------------------------
// Resolved-arm inspect runs synchronously.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn resolved_inspect_runs_sync_and_preserves_chain() {
    resolved_inspect_runs_sync_and_preserves_chain::main().await;
}

// ---------------------------------------------------------------------------
// Resolved-arm inspect returns Self, preserving the downstream chain.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn resolved_inspect_does_not_change_typestate() {
    resolved_inspect_does_not_change_typestate::main().await;
}

// ---------------------------------------------------------------------------
// InFlight-arm inspect defers closure until `.await`.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn inflight_inspect_runs_after_pending_resolves() {
    inflight_inspect_runs_after_pending_resolves::main().await;
}

// ---------------------------------------------------------------------------
// InFlight-arm inspect keeps the chain folding through downstream transitions.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn inflight_inspect_chains_through_subsequent_transitions() {
    inflight_inspect_chains_through_subsequent_transitions::main().await;
}
