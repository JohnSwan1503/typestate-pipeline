//! Contract for `examples/factory_ready_trait.rs`.

use typestate_pipeline::{No, Satisfiable, Satisfied, TypestateFactory, Yes};

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
    // Trait + method must exist with the documented signature.
    fn _ready_dispatch<F1: Satisfied, F2: Satisfiable>(
        bag: UserFactory<F1, F2>,
    ) -> User
    where
        UserFactory<F1, F2>: UserFactoryReady,
    {
        UserFactoryReady::into_finalized(bag)
    }
    let _: fn(UserFactory<Yes, No>) -> User = _ready_dispatch::<Yes, No>;
    let _: fn(UserFactory<Yes, Yes>) -> User = _ready_dispatch::<Yes, Yes>;
}

#[test]
fn surface_compiles() {
    surface_check();
}
