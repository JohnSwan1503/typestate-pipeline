//! Contract for `examples/factory_removable.rs`.

use typestate_pipeline::{No, TypestateFactory, Yes};

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required, removable)]
    name: String,
    #[field(required)]
    age: u32,
}

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    // drop_<field> — input bag has flag = Yes, output has flag = No.
    let _: fn(UserFactory<Yes, No>) -> UserFactory<No, No> = <UserFactory<Yes, No>>::drop_name;
    let _: fn(UserFactory<Yes, Yes>) -> UserFactory<No, Yes> = <UserFactory<Yes, Yes>>::drop_name;
}

#[test]
fn surface_compiles() {
    surface_check();
}
