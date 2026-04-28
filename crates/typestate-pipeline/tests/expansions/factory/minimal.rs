//! Contract for `examples/factory_minimal.rs`.
//!
//! Asserts the public surface the sketch advertises for `#[derive(TypestateFactory)]`
//! on a struct whose fields are all `required` with no other attributes.

use typestate_pipeline::{No, Satisfied, TypestateFactory, Yes};

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required)]
    name: String,
    #[field(required)]
    age: u32,
}

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    // Constructor — one impl on the all-`No` shape.
    let _: fn() -> UserFactory<No, No> = UserFactory::new;

    // Default impl on the all-`No` shape.
    let _: fn() -> UserFactory<No, No> = <UserFactory<No, No> as Default>::default;

    // Setter for `name` — flips F1 from No to Yes, leaves F2 free.
    let _: fn(UserFactory<No, No>, String) -> UserFactory<Yes, No> = <UserFactory<No, No>>::name;
    let _: fn(UserFactory<No, Yes>, String) -> UserFactory<Yes, Yes> = <UserFactory<No, Yes>>::name;

    // Setter for `age` — flips F2 from No to Yes, leaves F1 free.
    let _: fn(UserFactory<No, No>, u32) -> UserFactory<No, Yes> = <UserFactory<No, No>>::age;
    let _: fn(UserFactory<Yes, No>, u32) -> UserFactory<Yes, Yes> = <UserFactory<Yes, No>>::age;

    // Getters: applicable once the matching flag is `Yes`.
    let _: for<'a> fn(&'a UserFactory<Yes, No>) -> &'a String = <UserFactory<Yes, No>>::name;
    let _: for<'a> fn(&'a UserFactory<No, Yes>) -> &'a u32 = <UserFactory<No, Yes>>::age;

    // Finalize: callable once every required flag is `Yes`.
    let _: fn(UserFactory<Yes, Yes>) -> User = <UserFactory<Yes, Yes>>::finalize;

    // Companion `<Bag>Ready` trait — generic dispatch over the finalize-shaped
    // tuple. Compiling this fn at all witnesses the trait + impl exist.
    fn ready_dispatch<F1: Satisfied, F2: Satisfied>(bag: UserFactory<F1, F2>) -> User
    where
        UserFactory<F1, F2>: UserFactoryReady,
    {
        UserFactoryReady::into_finalized(bag)
    }
    let _: fn(UserFactory<Yes, Yes>) -> User = ready_dispatch::<Yes, Yes>;
}

#[test]
fn surface_compiles() {
    // Compilation of `surface_check` is the assertion; the call itself is
    // never reached at runtime in any meaningful way (it's just a no-op
    // sequence of fn-pointer coercions).
    surface_check();
}
