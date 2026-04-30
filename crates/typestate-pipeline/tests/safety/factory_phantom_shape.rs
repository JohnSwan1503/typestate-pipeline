#![allow(unused)]

#[path = "factory_phantom_shape/tests/all_internal_struct_finalizes_without_flag_generics.rs"]
pub mod all_internal_struct_finalizes_without_flag_generics;

#[path = "factory_phantom_shape/tests/singleton_flag_struct_round_trips.rs"]
pub mod singleton_flag_struct_round_trips;

#[path = "factory_phantom_shape/tests/one_flag_bag_is_send_and_sync_when_field_is.rs"]
pub mod one_flag_bag_is_send_and_sync_when_field_is;

#[path = "factory_phantom_shape/tests/many_flag_struct_round_trips.rs"]
pub mod many_flag_struct_round_trips;

// ---------------------------------------------------------------------------
// Zero flags — every field is `internal`. PhantomData<()>.
// ---------------------------------------------------------------------------

#[test]
fn all_internal_struct_finalizes_without_flag_generics() {
    all_internal_struct_finalizes_without_flag_generics::main();
}

// ---------------------------------------------------------------------------
// One flag — the trailing-comma singleton-tuple case. PhantomData<(F,)>.
// ---------------------------------------------------------------------------

#[test]
fn singleton_flag_struct_round_trips() {
    singleton_flag_struct_round_trips::main();
}

// ---------------------------------------------------------------------------
// Auto-trait forwarding through the singleton tuple. Compile-time check.
// ---------------------------------------------------------------------------

#[test]
fn one_flag_bag_is_send_and_sync_when_field_is() {
    one_flag_bag_is_send_and_sync_when_field_is::main();
}

// ---------------------------------------------------------------------------
// Many flags — control case.
// ---------------------------------------------------------------------------

#[test]
fn many_flag_struct_round_trips() {
    many_flag_struct_round_trips::main();
}
