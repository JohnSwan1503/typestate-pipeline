//! Contract for `examples/factory_default.rs`.

use typestate_pipeline::{No, Satisfiable, TypestateFactory, Yes};

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required)]
    name: String,
    #[field(default = 18)]
    age: u32,
}

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    let _: fn() -> UserFactory<No, No> = UserFactory::new;

    // Optional → `with_<field>` setter.
    let _: fn(UserFactory<No, No>, u32) -> UserFactory<No, Yes> =
        <UserFactory<No, No>>::with_age;

    // Default helper — no `val` parameter; applies the declared default.
    let _: fn(UserFactory<No, No>) -> UserFactory<No, Yes> =
        <UserFactory<No, No>>::age_default;
    let _: fn(UserFactory<Yes, No>) -> UserFactory<Yes, Yes> =
        <UserFactory<Yes, No>>::age_default;

    // finalize() resolves on BOTH age-flag states.
    let _: fn(UserFactory<Yes, No>) -> User = <UserFactory<Yes, No>>::finalize;
    let _: fn(UserFactory<Yes, Yes>) -> User = <UserFactory<Yes, Yes>>::finalize;

    // Generic dispatch over either age-flag state via Satisfiable.
    fn finalize_either<F2: Satisfiable>(bag: UserFactory<Yes, F2>) -> User {
        bag.finalize()
    }
    let _: fn(UserFactory<Yes, No>) -> User = finalize_either::<No>;
    let _: fn(UserFactory<Yes, Yes>) -> User = finalize_either::<Yes>;
}

#[test]
fn surface_compiles() {
    surface_check();
}
