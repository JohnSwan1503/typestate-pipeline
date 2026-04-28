//! Contract for `examples/factory_setter_input_type.rs`.

use typestate_pipeline::{No, TypestateFactory, Yes};

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct Profile {
    #[field(required)]
    name: String,
    #[field(default = None, setter = wrap_some, input = String)]
    worker: Option<String>,
}

fn wrap_some(s: String) -> Option<String> {
    Some(s)
}

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    let _: fn() -> ProfileFactory<No, No> = ProfileFactory::new;

    // Setter takes the *input* type (`String`), not the storage type.
    let _: fn(ProfileFactory<No, No>, String) -> ProfileFactory<No, Yes> =
        <ProfileFactory<No, No>>::with_worker;

    // Default helper takes no args and writes the storage type directly.
    let _: fn(ProfileFactory<No, No>) -> ProfileFactory<No, Yes> =
        <ProfileFactory<No, No>>::worker_default;
}

#[test]
fn surface_compiles() {
    surface_check();
}
