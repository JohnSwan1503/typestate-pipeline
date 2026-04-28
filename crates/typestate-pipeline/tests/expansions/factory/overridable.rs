//! Contract for `examples/factory_overridable.rs`.

use typestate_pipeline::{TypestateFactory, Yes};

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required, overridable)]
    name: String,
}

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    // override_<field> — input bag's flag is Yes, output stays Yes.
    let _: fn(UserFactory<Yes>, String) -> UserFactory<Yes> =
        <UserFactory<Yes>>::override_name;
}

#[test]
fn surface_compiles() {
    surface_check();
}
