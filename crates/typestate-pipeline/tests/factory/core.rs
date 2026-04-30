#![allow(unused)]

#[path = "core/tests/shared.rs"]
pub mod shared;

#[path = "core/tests/build_in_order.rs"]
pub mod build_in_order;

#[path = "core/tests/build_in_arbitrary_order.rs"]
pub mod build_in_arbitrary_order;

#[path = "core/tests/default_helper_fills_in_optional.rs"]
pub mod default_helper_fills_in_optional;

#[path = "core/tests/getter_borrows_set_field.rs"]
pub mod getter_borrows_set_field;

#[path = "core/tests/empty_bag_dropped_does_not_touch_uninit_fields.rs"]
pub mod empty_bag_dropped_does_not_touch_uninit_fields;

#[path = "core/tests/partial_bag_dropped_drops_only_set_fields.rs"]
pub mod partial_bag_dropped_drops_only_set_fields;

#[path = "core/tests/fully_populated_bag_dropped_drops_all.rs"]
pub mod fully_populated_bag_dropped_drops_all;

#[path = "core/tests/finalize_does_not_double_drop.rs"]
pub mod finalize_does_not_double_drop;

#[path = "core/tests/drop_field_drops_value_once.rs"]
pub mod drop_field_drops_value_once;

#[path = "core/tests/drop_field_then_reset_doesnt_double_drop.rs"]
pub mod drop_field_then_reset_doesnt_double_drop;

#[path = "core/tests/override_drops_old_value.rs"]
pub mod override_drops_old_value;

#[path = "core/tests/finalize_uses_defaults_when_optional_no.rs"]
pub mod finalize_uses_defaults_when_optional_no;

#[path = "core/tests/finalize_keeps_explicit_values_when_optional_yes.rs"]
pub mod finalize_keeps_explicit_values_when_optional_yes;

#[path = "core/tests/finalize_mixes_set_and_default.rs"]
pub mod finalize_mixes_set_and_default;

#[path = "core/tests/custom_bag_name.rs"]
pub mod custom_bag_name;

#[path = "core/tests/custom_setter_name.rs"]
pub mod custom_setter_name;

#[path = "core/tests/custom_transformer_fn.rs"]
pub mod custom_transformer_fn;

#[path = "core/tests/fallible_transformer_success.rs"]
pub mod fallible_transformer_success;

#[path = "core/tests/fallible_transformer_failure.rs"]
pub mod fallible_transformer_failure;

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
fn getter_borrows_set_field() {
    getter_borrows_set_field::main();
}

// ===========================================================================
// Drop semantics — partially-populated and fully-populated bags.
// ===========================================================================

#[test]
fn empty_bag_dropped_does_not_touch_uninit_fields() {
    empty_bag_dropped_does_not_touch_uninit_fields::main();
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
// `#[field(removable)]` — drop_<field> transitions Yes → No, drops once.
// ===========================================================================

#[test]
fn drop_field_drops_value_once() {
    drop_field_drops_value_once::main();
}

#[test]
fn drop_field_then_reset_doesnt_double_drop() {
    drop_field_then_reset_doesnt_double_drop::main();
}

// ===========================================================================
// `#[field(overridable)]` — override_<field> stays in Yes, drops the old.
// ===========================================================================

#[test]
fn override_drops_old_value() {
    override_drops_old_value::main();
}

// ===========================================================================
// Conditional finalize — optional+default may be Yes OR No at finalize.
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
// Custom names — `#[factory(name = …)]`, `#[field(name = …)]`.
// ===========================================================================

#[test]
fn custom_bag_name() {
    custom_bag_name::main();
}

#[test]
fn custom_setter_name() {
    custom_setter_name::main();
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
