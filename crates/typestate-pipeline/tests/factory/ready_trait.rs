#![allow(unused)]

#[path = "ready_trait/tests/shared.rs"]
mod shared;

pub(self) use shared::*;

#[path = "ready_trait/tests/ready_trait_is_implemented_when_required_flags_yes.rs"]
pub mod ready_trait_is_implemented_when_required_flags_yes;

#[path = "ready_trait/tests/ready_trait_works_when_optional_set_too.rs"]
pub mod ready_trait_works_when_optional_set_too;

#[path = "ready_trait/tests/dispatch_via_trait_matches_inherent_finalize.rs"]
pub mod dispatch_via_trait_matches_inherent_finalize;

// ---------------------------------------------------------------------------
// Trait implemented when required flags are Yes (optional may stay No).
// ---------------------------------------------------------------------------

#[test]
fn ready_trait_is_implemented_when_required_flags_yes() {
    ready_trait_is_implemented_when_required_flags_yes::main();
}

// ---------------------------------------------------------------------------
// Trait still applies when the optional fields are explicitly set.
// ---------------------------------------------------------------------------

#[test]
fn ready_trait_works_when_optional_set_too() {
    ready_trait_works_when_optional_set_too::main();
}

// ---------------------------------------------------------------------------
// Trait dispatch matches inherent finalize exactly.
// ---------------------------------------------------------------------------

#[test]
fn dispatch_via_trait_matches_inherent_finalize() {
    dispatch_via_trait_matches_inherent_finalize::main();
}
