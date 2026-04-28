//! Contract for `examples/factory_setter_transformer.rs`.

use typestate_pipeline::{No, TypestateFactory, Yes};

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required, setter = trim_name)]
    name: String,
}

fn trim_name(value: String) -> String {
    value.trim().to_owned()
}

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    let _: fn() -> UserFactory<No> = UserFactory::new;
    // Setter signature is identical to baseline — transformer is internal.
    let _: fn(UserFactory<No>, String) -> UserFactory<Yes> = <UserFactory<No>>::name;
    let _: fn(UserFactory<Yes>) -> User = <UserFactory<Yes>>::finalize;
}

#[test]
fn surface_compiles() {
    surface_check();
}
