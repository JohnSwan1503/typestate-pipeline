#![allow(unused)]

#[path = "input_type/tests/shared.rs"]
mod shared;

use shared::*;

#[path = "input_type/tests/setter_takes_input_type_not_field_type.rs"]
pub mod setter_takes_input_type_not_field_type;

#[path = "input_type/tests/default_helper_bypasses_transformer.rs"]
pub mod default_helper_bypasses_transformer;

#[path = "input_type/tests/unset_default_field_uses_default_at_finalize.rs"]
pub mod unset_default_field_uses_default_at_finalize;

// ---------------------------------------------------------------------------
// User-facing setter accepts the input type, not the storage type.
// ---------------------------------------------------------------------------

#[test]
fn setter_takes_input_type_not_field_type() {
    setter_takes_input_type_not_field_type::main();
}

// ---------------------------------------------------------------------------
// `<field>_default()` writes the storage type directly, bypassing the setter
// transformer.
// ---------------------------------------------------------------------------

#[test]
fn default_helper_bypasses_transformer() {
    default_helper_bypasses_transformer::main();
}

// ---------------------------------------------------------------------------
// Unset field with `default = ...` resolves at finalize.
// ---------------------------------------------------------------------------

#[test]
fn unset_default_field_uses_default_at_finalize() {
    unset_default_field_uses_default_at_finalize::main();
}
