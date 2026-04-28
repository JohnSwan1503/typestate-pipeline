//! Contract for `examples/factory_required_and_optional.rs`.

use typestate_pipeline::{No, TypestateFactory, Yes};

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required)]
    name: String,
    #[field(optional)]
    nickname: String,
}

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    let _: fn() -> UserFactory<No, No> = UserFactory::new;

    // Required → bare-named setter.
    let _: fn(UserFactory<No, No>, String) -> UserFactory<Yes, No> =
        <UserFactory<No, No>>::name;

    // Optional → `with_<field>` setter; same flag transition (No -> Yes).
    let _: fn(UserFactory<No, No>, String) -> UserFactory<No, Yes> =
        <UserFactory<No, No>>::with_nickname;

    // Both flags still required at finalize — `optional` did not relax this.
    let _: fn(UserFactory<Yes, Yes>) -> User = <UserFactory<Yes, Yes>>::finalize;
}

#[test]
fn surface_compiles() {
    surface_check();
}
