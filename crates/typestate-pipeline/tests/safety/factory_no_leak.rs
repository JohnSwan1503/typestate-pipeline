#![allow(unused)]

#[path = "factory_no_leak/tests/error.rs"]
pub mod error;

#[path = "factory_no_leak/tests/bookkeeping.rs"]
pub mod bookkeeping;

#[path = "factory_no_leak/tests/fallible_setter_failure_drops_other_set_fields.rs"]
pub mod fallible_setter_failure_drops_other_set_fields;

#[path = "factory_no_leak/tests/fallible_overrider_failure_drops_other_set_fields.rs"]
pub mod fallible_overrider_failure_drops_other_set_fields;

#[path = "factory_no_leak/tests/async_setter_dropped_mid_await_drops_other_set_fields.rs"]
pub mod async_setter_dropped_mid_await_drops_other_set_fields;

// ---------------------------------------------------------------------------
// Sync fallible setter — the simplest manifestation of the bug.
// ---------------------------------------------------------------------------

#[test]
pub fn fallible_setter_failure_drops_other_set_fields() {
    fallible_setter_failure_drops_other_set_fields::main();
}

// ---------------------------------------------------------------------------
// Sync fallible overrider — worst case: pre-fix, the OLD value was already
// `assume_init_drop`-ed before the transformer ran, so failure both leaked
// other fields *and* destroyed the old field with no replacement.
// ---------------------------------------------------------------------------

#[test]
pub fn fallible_overrider_failure_drops_other_set_fields() {
    fallible_overrider_failure_drops_other_set_fields::main();
}

// ---------------------------------------------------------------------------
// Async setter, future dropped mid-await — the bug also bites without an
// explicit `?`. We hand-poll once to suspend at the inner await, then drop
// the future. With the fix, `self` was never moved into `ManuallyDrop` so
// dropping the future drops `self` normally.
// ---------------------------------------------------------------------------

#[test]
pub fn async_setter_dropped_mid_await_drops_other_set_fields() {
    async_setter_dropped_mid_await_drops_other_set_fields::main();
}
