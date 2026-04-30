//! Contract for `examples/expansions/factory/empty_alias.rs`.

use typestate_pipeline::{No, TypestateFactory};

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
    // The alias must exist and resolve to the all-`No` shape. If the
    // codegen renames the alias or changes its arity, the type-equality
    // check below fails to compile.
    let _: fn() -> UserFactoryEmpty = || UserFactory::new();
    let _: fn() -> UserFactory<No, No> = || -> UserFactoryEmpty { UserFactory::new() };
}

#[test]
fn surface_compiles() {
    surface_check();
}
