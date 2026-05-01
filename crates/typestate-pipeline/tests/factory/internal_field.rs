#![allow(unused)]

#[path = "internal_field/tests/shared.rs"]
mod shared;

use shared::*;

#[path = "internal_field/tests/constructor_takes_internal_field_as_argument.rs"]
pub mod constructor_takes_internal_field_as_argument;

#[path = "internal_field/tests/internal_getter_is_unconditional.rs"]
pub mod internal_getter_is_unconditional;

#[path = "internal_field/tests/internal_field_dropped_from_flag_generic_list.rs"]
pub mod internal_field_dropped_from_flag_generic_list;

#[path = "internal_field/tests/pipeline_arm_works_for_non_internal_fields.rs"]
pub mod pipeline_arm_works_for_non_internal_fields;

#[path = "internal_field/tests/finalize_passes_internal_field_through.rs"]
pub mod finalize_passes_internal_field_through;

#[path = "internal_field/tests/carrier_internal_getter_is_unconditional.rs"]
pub mod carrier_internal_getter_is_unconditional;

#[path = "internal_field/tests/carrier_non_internal_getter_gates_on_yes_flag.rs"]
pub mod carrier_non_internal_getter_gates_on_yes_flag;

// ---------------------------------------------------------------------------
// `Factory::new(...)` accepts the internal field positionally.
// ---------------------------------------------------------------------------

#[test]
fn constructor_takes_internal_field_as_argument() {
    constructor_takes_internal_field_as_argument::main();
}

// ---------------------------------------------------------------------------
// Internal getter callable on any bag shape.
// ---------------------------------------------------------------------------

#[test]
fn internal_getter_is_unconditional() {
    internal_getter_is_unconditional::main();
}

// ---------------------------------------------------------------------------
// Internal field doesn't appear in the bag's flag generic list.
// ---------------------------------------------------------------------------

#[test]
fn internal_field_dropped_from_flag_generic_list() {
    internal_field_dropped_from_flag_generic_list::main();
}

// ---------------------------------------------------------------------------
// Pipeline-arm setters work normally for non-internal fields.
// ---------------------------------------------------------------------------

#[test]
fn pipeline_arm_works_for_non_internal_fields() {
    pipeline_arm_works_for_non_internal_fields::main();
}

// ---------------------------------------------------------------------------
// `finalize` reads the internal field from plain-`T` storage.
// ---------------------------------------------------------------------------

#[test]
fn finalize_passes_internal_field_through() {
    finalize_passes_internal_field_through::main();
}

// ---------------------------------------------------------------------------
// Carrier-arm internal getter has no flag bound.
// ---------------------------------------------------------------------------

#[test]
fn carrier_internal_getter_is_unconditional() {
    carrier_internal_getter_is_unconditional::main();
}

// ---------------------------------------------------------------------------
// Carrier-arm non-internal getter requires the field's flag = `Yes`.
// ---------------------------------------------------------------------------

#[test]
fn carrier_non_internal_getter_gates_on_yes_flag() {
    carrier_non_internal_getter_gates_on_yes_flag::main();
}
