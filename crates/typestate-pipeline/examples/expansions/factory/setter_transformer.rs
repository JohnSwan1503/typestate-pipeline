//! `#[field(setter = my_fn)]` runs `my_fn(val)` inside the setter and
//! stores its return. Useful for normalization (trim, lowercase, parse,
//! etc.). The setter's *input* type is still the field type — only the
//! *storage* routes through the transformer. For a different input type,
//! see `./setter_input_type.rs`.
//!
//! =============================================================================
//! Generated (sketch) — diff from baseline (see `./minimal.rs`)
//! =============================================================================
//!
//!     impl UserFactory<No> {
//!         pub fn name(self, val: String) -> UserFactory<Yes> {
//!             // body stores `trim_name(val)`
//!         }
//!     }
//!
//! The setter signature is identical to the baseline; only the body
//! differs.

use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required, setter = trim_name)]
    name: String,
}

fn trim_name(value: String) -> String {
    value.trim().to_owned()
}

fn main() {
    let user = UserFactory::new()
        .name("   Alice   ".to_owned())
        .finalize();
    assert_eq!(user.name, "Alice");
}
