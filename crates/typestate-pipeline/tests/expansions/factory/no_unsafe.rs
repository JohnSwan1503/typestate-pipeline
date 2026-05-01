//! Contract for `examples/factory_no_unsafe.rs`. Public surface is
//! identical to baseline; this contract is the explicit witness that
//! enabling `no_unsafe` does not change names or signatures.

use typestate_pipeline::{No, Storage, TypestateFactory, Yes};

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
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
    let _: fn(UserFactory<No, No>, String) -> UserFactory<Yes, No> = <UserFactory<No, No>>::name;
    let _: fn(UserFactory<No, No>, u32) -> UserFactory<No, Yes> = <UserFactory<No, No>>::with_age;
    let _: fn(UserFactory<No, No>) -> UserFactory<No, Yes> = <UserFactory<No, No>>::age_default;

    // finalize: in safe mode the required slot is pinned to concrete `Yes`
    // and the optional-with-default slot is bounded by `Storage<T>`.
    fn finalize_either<F2: Storage<u32>>(bag: UserFactory<Yes, F2>) -> User {
        bag.finalize()
    }
    let _: fn(UserFactory<Yes, No>) -> User = finalize_either::<No>;
    let _: fn(UserFactory<Yes, Yes>) -> User = finalize_either::<Yes>;
}

#[test]
fn surface_compiles() {
    surface_check();
}
