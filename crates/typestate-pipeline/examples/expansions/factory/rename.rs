//! Renaming knobs:
//!
//! - `#[factory(name = MyType)]` overrides the generated bag type's name
//!   (default `<Original>Factory`).
//! - `#[field(name = my_setter)]` overrides the setter's method name. The
//!   default-helper name (`<field>_default`) and getter name (`<field>`)
//!   are not affected. (Use `#[field(default_helper = my_helper)]` to
//!   rename the helper.)
//!
//! =============================================================================
//! Generated (sketch) — diff from baseline (see `./minimal.rs`)
//! =============================================================================
//!
//!     // `#[factory(name = ManifestBuilder)]` renames the bag type:
//!     struct ManifestBuilder<F1 = No> { /* private */ }
//!
//!     // `#[field(name = shout_title)]` renames the setter:
//!     impl ManifestBuilder<No> {
//!         pub fn shout_title(self, val: String) -> ManifestBuilder<Yes>;
//!     }
//!     // Getter still resolves under the field name (`title`), not the setter name.
//!     impl ManifestBuilder<Yes> { pub fn title(&self) -> &String; }

use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(name = ManifestBuilder)]
#[allow(dead_code)]
struct Manifest {
    #[field(required, name = shout_title)]
    title: String,
}

fn main() {
    let bag = ManifestBuilder::new().shout_title("DATASET-X".to_owned());
    assert_eq!(bag.title(), "DATASET-X"); // getter name unchanged
    let m = bag.finalize();
    assert_eq!(m.title, "DATASET-X");
}
