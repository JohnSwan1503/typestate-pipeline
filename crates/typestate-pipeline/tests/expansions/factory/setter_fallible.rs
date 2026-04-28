//! Contract for `examples/factory_setter_fallible.rs`.

use typestate_pipeline::{No, TypestateFactory, Yes};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ValidationError(&'static str);

#[derive(TypestateFactory)]
#[factory(error = ValidationError)]
#[allow(dead_code)]
struct User {
    #[field(required, setter = require_nonempty, fallible)]
    name: String,
}

fn require_nonempty(value: String) -> Result<String, ValidationError> {
    Ok(value)
}

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    let _: fn() -> UserFactory<No> = UserFactory::new;

    // Fallible setter returns `Result<NextBag, ValidationError>`.
    let _: fn(UserFactory<No>, String) -> Result<UserFactory<Yes>, ValidationError> =
        <UserFactory<No>>::name;

    let _: fn(UserFactory<Yes>) -> User = <UserFactory<Yes>>::finalize;
}

#[test]
fn surface_compiles() {
    surface_check();
}
