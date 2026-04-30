#![allow(unused)]

#[path = "factory_pipeline/tests/shared.rs"]
pub mod shared;

#[path = "factory_pipeline/tests/pipeline_setters_chain_in_resolved_mode.rs"]
pub mod pipeline_setters_chain_in_resolved_mode;

#[path = "factory_pipeline/tests/pipeline_setters_chain_through_inflight.rs"]
pub mod pipeline_setters_chain_through_inflight;

#[path = "factory_pipeline/tests/pipeline_drop_field_transitions_yes_to_no.rs"]
pub mod pipeline_drop_field_transitions_yes_to_no;

#[path = "factory_pipeline/tests/pipeline_override_replaces_value.rs"]
pub mod pipeline_override_replaces_value;

// ---------------------------------------------------------------------------
// Resolved-mode setter chain reaches the follow-on phase.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn pipeline_setters_chain_in_resolved_mode() {
    pipeline_setters_chain_in_resolved_mode::main().await;
}

// ---------------------------------------------------------------------------
// InFlight-mode setter chain folds the bag's fallible Result into the pending
// future — no Result at the call site.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn pipeline_setters_chain_through_inflight() {
    pipeline_setters_chain_through_inflight::main().await;
}

// ---------------------------------------------------------------------------
// `drop_<field>` flips the carrier-side flag Yes -> No so the field can be
// re-set without an override.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn pipeline_drop_field_transitions_yes_to_no() {
    pipeline_drop_field_transitions_yes_to_no::main().await;
}

// ---------------------------------------------------------------------------
// `override_<field>` keeps the flag at Yes and replaces the stored value.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn pipeline_override_replaces_value() {
    pipeline_override_replaces_value::main().await;
}
