#![cfg(feature = "no_unsafe")]
#![allow(unused)]

#[path = "factory_no_unsafe/tests/shared.rs"]
pub mod shared;

#[path = "factory_no_unsafe/tests/build_in_order.rs"]
pub mod build_in_order;

#[path = "factory_no_unsafe/tests/build_in_arbitrary_order.rs"]
pub mod build_in_arbitrary_order;

#[path = "factory_no_unsafe/tests/default_helper_fills_in_optional.rs"]
pub mod default_helper_fills_in_optional;

#[path = "factory_no_unsafe/tests/finalize_uses_default_when_optional_unset.rs"]
pub mod finalize_uses_default_when_optional_unset;

#[path = "factory_no_unsafe/tests/getter_borrows_set_field.rs"]
pub mod getter_borrows_set_field;

#[path = "factory_no_unsafe/tests/empty_bag_dropped_does_not_touch_unset_fields.rs"]
pub mod empty_bag_dropped_does_not_touch_unset_fields;

#[path = "factory_no_unsafe/tests/partial_bag_dropped_drops_only_set_fields.rs"]
pub mod partial_bag_dropped_drops_only_set_fields;

#[path = "factory_no_unsafe/tests/fully_populated_bag_dropped_drops_all.rs"]
pub mod fully_populated_bag_dropped_drops_all;

#[path = "factory_no_unsafe/tests/finalize_does_not_double_drop.rs"]
pub mod finalize_does_not_double_drop;

#[path = "factory_no_unsafe/tests/drop_field_drops_value_once.rs"]
pub mod drop_field_drops_value_once;

#[path = "factory_no_unsafe/tests/drop_field_then_reset_doesnt_double_drop.rs"]
pub mod drop_field_then_reset_doesnt_double_drop;

#[path = "factory_no_unsafe/tests/override_drops_old_value.rs"]
pub mod override_drops_old_value;

#[path = "factory_no_unsafe/tests/finalize_uses_defaults_when_optional_no.rs"]
pub mod finalize_uses_defaults_when_optional_no;

#[path = "factory_no_unsafe/tests/finalize_keeps_explicit_values_when_optional_yes.rs"]
pub mod finalize_keeps_explicit_values_when_optional_yes;

#[path = "factory_no_unsafe/tests/finalize_mixes_set_and_default.rs"]
pub mod finalize_mixes_set_and_default;

#[path = "factory_no_unsafe/tests/custom_transformer_fn.rs"]
pub mod custom_transformer_fn;

#[path = "factory_no_unsafe/tests/fallible_transformer_success.rs"]
pub mod fallible_transformer_success;

#[path = "factory_no_unsafe/tests/fallible_transformer_failure.rs"]
pub mod fallible_transformer_failure;

#[path = "factory_no_unsafe/tests/fallible_setter_failure_drops_other_set_fields.rs"]
pub mod fallible_setter_failure_drops_other_set_fields;

#[path = "factory_no_unsafe/tests/fallible_overrider_failure_drops_other_set_fields.rs"]
pub mod fallible_overrider_failure_drops_other_set_fields;

#[path = "factory_no_unsafe/tests/async_setter_dropped_mid_await_drops_other_set_fields.rs"]
pub mod async_setter_dropped_mid_await_drops_other_set_fields;

#[path = "factory_no_unsafe/tests/internal_field_round_trips.rs"]
pub mod internal_field_round_trips;

// ===========================================================================
// Construction, ordering, getters, default helper.
// ===========================================================================

#[test]
fn build_in_order() {
    build_in_order::main();
}

#[test]
fn build_in_arbitrary_order() {
    build_in_arbitrary_order::main();
}

#[test]
fn default_helper_fills_in_optional() {
    default_helper_fills_in_optional::main();
}

#[test]
fn finalize_uses_default_when_optional_unset() {
    finalize_uses_default_when_optional_unset::main();
}

#[test]
fn getter_borrows_set_field() {
    getter_borrows_set_field::main();
}

// ===========================================================================
// Drop semantics — auto-Drop on the sister-struct shape.
// ===========================================================================

#[test]
fn empty_bag_dropped_does_not_touch_unset_fields() {
    empty_bag_dropped_does_not_touch_unset_fields::main();
}

#[test]
fn partial_bag_dropped_drops_only_set_fields() {
    partial_bag_dropped_drops_only_set_fields::main();
}

#[test]
fn fully_populated_bag_dropped_drops_all() {
    fully_populated_bag_dropped_drops_all::main();
}

#[test]
fn finalize_does_not_double_drop() {
    finalize_does_not_double_drop::main();
}

// ===========================================================================
// Removable / overridable.
// ===========================================================================

#[test]
fn drop_field_drops_value_once() {
    drop_field_drops_value_once::main();
}

#[test]
fn drop_field_then_reset_doesnt_double_drop() {
    drop_field_then_reset_doesnt_double_drop::main();
}

#[test]
fn override_drops_old_value() {
    override_drops_old_value::main();
}

// ===========================================================================
// Conditional finalize via Storage::finalize_or.
// ===========================================================================

#[test]
fn finalize_uses_defaults_when_optional_no() {
    finalize_uses_defaults_when_optional_no::main();
}

#[test]
fn finalize_keeps_explicit_values_when_optional_yes() {
    finalize_keeps_explicit_values_when_optional_yes::main();
}

#[test]
fn finalize_mixes_set_and_default() {
    finalize_mixes_set_and_default::main();
}

// ===========================================================================
// Custom transformers (sync infallible, sync fallible).
// ===========================================================================

#[test]
fn custom_transformer_fn() {
    custom_transformer_fn::main();
}

#[test]
fn fallible_transformer_success() {
    fallible_transformer_success::main();
}

#[test]
fn fallible_transformer_failure() {
    fallible_transformer_failure::main();
}

// ===========================================================================
// Leak-safety on transformer / cancellation paths.
// ===========================================================================

#[test]
fn fallible_setter_failure_drops_other_set_fields() {
    fallible_setter_failure_drops_other_set_fields::main();
}

#[test]
fn fallible_overrider_failure_drops_other_set_fields() {
    fallible_overrider_failure_drops_other_set_fields::main();
}

#[test]
fn async_setter_dropped_mid_await_drops_other_set_fields() {
    async_setter_dropped_mid_await_drops_other_set_fields::main();
}

// ===========================================================================
// Internal field round-trip.
// ===========================================================================

#[test]
fn internal_field_round_trips() {
    internal_field_round_trips::main();
}
