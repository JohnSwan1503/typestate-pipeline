#![cfg(feature = "dataset-authoring-example")]
#![allow(unused)]

#[path = "dataset_authoring/tests/shared.rs"]
pub mod shared;

#[path = "dataset_authoring/tests/new_evm_rpc_flow_terminates_at_deployed.rs"]
pub mod new_evm_rpc_flow_terminates_at_deployed;

#[path = "dataset_authoring/tests/new_derived_flow_chains_through_single_await.rs"]
pub mod new_derived_flow_chains_through_single_await;

#[path = "dataset_authoring/tests/bump_patch_increments_existing_version.rs"]
pub mod bump_patch_increments_existing_version;

#[path = "dataset_authoring/tests/bump_patch_errors_when_no_prior_version.rs"]
pub mod bump_patch_errors_when_no_prior_version;

#[path = "dataset_authoring/tests/edit_existing_kind_mismatch_surfaces_error.rs"]
pub mod edit_existing_kind_mismatch_surfaces_error;

// ---------------------------------------------------------------------------
// EVM-RPC happy path lands at Deployed.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn new_evm_rpc_flow_terminates_at_deployed() {
    new_evm_rpc_flow_terminates_at_deployed::main().await;
}

// ---------------------------------------------------------------------------
// Derived flow folds a multi-async chain into one terminal `.await?`.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn new_derived_flow_chains_through_single_await() {
    new_derived_flow_chains_through_single_await::main().await;
}

// ---------------------------------------------------------------------------
// `bump_patch()` arithmetically increments the existing version.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn bump_patch_increments_existing_version() {
    bump_patch_increments_existing_version::main().await;
}

// ---------------------------------------------------------------------------
// `bump_patch()` against an untagged dataset surfaces NoPriorVersion.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn bump_patch_errors_when_no_prior_version() {
    bump_patch_errors_when_no_prior_version::main().await;
}

// ---------------------------------------------------------------------------
// Editing a dataset of the wrong kind surfaces KindMismatch.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn edit_existing_kind_mismatch_surfaces_error() {
    edit_existing_kind_mismatch_surfaces_error::main().await;
}
