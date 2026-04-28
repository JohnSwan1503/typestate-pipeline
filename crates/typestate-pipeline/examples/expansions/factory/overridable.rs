//! `#[field(overridable)]` emits an `override_<field>(self, val)` method
//! that consumes the bag, drops the prior value, stores the new one, and
//! returns a bag whose flag is still `Yes` (the bag *stays* in the set
//! state — only the value changes).
//!
//! Unlike a setter (which requires the input bag's flag to be `No`),
//! `override_<field>` requires the input bag's flag to be `Yes`. Combine
//! with `fallible` / `async_fn` exactly as for setters.
//!
//! =============================================================================
//! Generated (sketch) — diff from baseline (see `./minimal.rs`)
//! =============================================================================
//!
//!     impl UserFactory<Yes> {
//!         pub fn override_name(self, val: String) -> UserFactory<Yes>; // Yes -> Yes
//!     }

use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required, overridable)]
    name: String,
}

fn main() {
    // Setter: No -> Yes; override_: Yes -> Yes.
    let user = UserFactory::new()
        .name("draft".to_owned())
        .override_name("Alice".to_owned())
        .finalize();
    assert_eq!(user.name, "Alice");
}
