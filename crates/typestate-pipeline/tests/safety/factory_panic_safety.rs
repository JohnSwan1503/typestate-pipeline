#![allow(unused)]

#[path = "factory_panic_safety/tests/shared.rs"]
mod shared;

use shared::*;

#[path = "factory_panic_safety/tests/panic_in_drop_still_drops_subsequent_fields.rs"]
pub mod panic_in_drop_still_drops_subsequent_fields;

#[path = "factory_panic_safety/tests/panic_in_default_expr_during_finalize_drops_already_read_fields.rs"]
pub mod panic_in_default_expr_during_finalize_drops_already_read_fields;

#[path = "factory_panic_safety/tests/panic_in_old_value_drop_during_override_drops_other_fields.rs"]
pub mod panic_in_old_value_drop_during_override_drops_other_fields;

#[path = "factory_panic_safety/tests/panic_in_old_value_drop_during_remove_drops_other_fields.rs"]
pub mod panic_in_old_value_drop_during_remove_drops_other_fields;

// ---------------------------------------------------------------------------
// Bag's Drop: a panic in field N's T::drop must not leak fields N+1..end.
// ---------------------------------------------------------------------------

#[test]
fn panic_in_drop_still_drops_subsequent_fields() {
    panic_in_drop_still_drops_subsequent_fields::main();
}

// ---------------------------------------------------------------------------
// finalize(): a panic in `default = …` must not leak fields after it.
// ---------------------------------------------------------------------------

#[test]
fn panic_in_default_expr_during_finalize_drops_already_read_fields() {
    panic_in_default_expr_during_finalize_drops_already_read_fields::main();
}

// ---------------------------------------------------------------------------
// override_<field>: a panic in OLD value's T::drop must not leak others.
// ---------------------------------------------------------------------------

#[test]
fn panic_in_old_value_drop_during_override_drops_other_fields() {
    panic_in_old_value_drop_during_override_drops_other_fields::main();
}

// ---------------------------------------------------------------------------
// drop_<field>: a panic in OLD value's T::drop must not leak others.
// ---------------------------------------------------------------------------

#[test]
fn panic_in_old_value_drop_during_remove_drops_other_fields() {
    panic_in_old_value_drop_during_remove_drops_other_fields::main();
}
