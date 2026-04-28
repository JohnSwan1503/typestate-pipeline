//! `#[field(removable)]` emits a `drop_<field>(self)` method that consumes
//! the bag, drops the stored value, and returns a bag whose flag is back
//! to `No`. The user can then call the setter again to re-supply.
//!
//! =============================================================================
//! Generated (sketch) — diff from baseline (see `./minimal.rs`)
//! =============================================================================
//!
//!     impl<F2> UserFactory<Yes, F2> {
//!         pub fn drop_name(self) -> UserFactory<No, F2>;  // Yes -> No
//!     }

use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required, removable)]
    name: String,
    #[field(required)]
    age: u32,
}

fn main() {
    let bag = UserFactory::new().name("draft".to_owned()).age(30);

    // Drop just the name; flag goes Yes -> No.
    let bag = bag.drop_name();

    // Set it again with the final value; flag goes No -> Yes.
    let user = bag.name("Alice".to_owned()).finalize();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.age, 30);
}
