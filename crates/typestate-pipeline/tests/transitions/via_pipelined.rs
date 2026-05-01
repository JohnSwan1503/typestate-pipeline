#![allow(unused)]

#[path = "via_pipelined/tests/shared.rs"]
mod shared;

pub(self) use shared::*;

#[path = "via_pipelined/tests/transitions_chain_without_error_arg.rs"]
pub mod transitions_chain_without_error_arg;

#[path = "via_pipelined/tests/transitions_chain_propagates_error.rs"]
pub mod transitions_chain_propagates_error;

#[path = "via_pipelined/tests/factory_pipeline_arms_without_error_arg.rs"]
pub mod factory_pipeline_arms_without_error_arg;

#[path = "via_pipelined/tests/intofuture_provided_by_pipelined.rs"]
pub mod intofuture_provided_by_pipelined;

// ---------------------------------------------------------------------------
// `#[transitions]` chain compiles without `error =` (read from Pipelined).
// ---------------------------------------------------------------------------

#[tokio::test]
async fn transitions_chain_without_error_arg() {
    transitions_chain_without_error_arg::main().await;
}

// ---------------------------------------------------------------------------
// Errors propagate through the inferred error type.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn transitions_chain_propagates_error() {
    transitions_chain_propagates_error::main().await;
}

// ---------------------------------------------------------------------------
// Factory pipeline arms compile without `error =` (no fallible setters).
// ---------------------------------------------------------------------------

#[tokio::test]
async fn factory_pipeline_arms_without_error_arg() {
    factory_pipeline_arms_without_error_arg::main().await;
}

// ---------------------------------------------------------------------------
// `pipelined!` supplies the carrier's `IntoFuture` impl.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn intofuture_provided_by_pipelined() {
    intofuture_provided_by_pipelined::main().await;
}
