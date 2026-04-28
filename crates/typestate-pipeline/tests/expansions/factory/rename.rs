//! Contract for `examples/factory_rename.rs`.

use typestate_pipeline::{No, TypestateFactory, Yes};

#[derive(TypestateFactory)]
#[factory(name = ManifestBuilder)]
#[allow(dead_code)]
struct Manifest {
    #[field(required, name = shout_title)]
    title: String,
}

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    // Bag type is `ManifestBuilder`, not `ManifestFactory`.
    let _: fn() -> ManifestBuilder<No> = ManifestBuilder::new;

    // Setter is `shout_title`, not `title`.
    let _: fn(ManifestBuilder<No>, String) -> ManifestBuilder<Yes> =
        <ManifestBuilder<No>>::shout_title;

    // Getter is still under the field name.
    let _: for<'a> fn(&'a ManifestBuilder<Yes>) -> &'a String =
        <ManifestBuilder<Yes>>::title;

    let _: fn(ManifestBuilder<Yes>) -> Manifest = <ManifestBuilder<Yes>>::finalize;
}

#[test]
fn surface_compiles() {
    surface_check();
}
