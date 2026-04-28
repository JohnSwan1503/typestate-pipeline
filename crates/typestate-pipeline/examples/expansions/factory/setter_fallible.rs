//! `#[field(setter = my_fn, fallible)]` makes the setter return
//! `Result<NextBag, Error>`. The error type is read from
//! `#[factory(error = Error)]`, which is required when any field is
//! `fallible`. A failing setter does not consume `self` past the point of
//! failure — the bag still owns its previously-set fields and they drop
//! normally on the `Err` branch.
//!
//! =============================================================================
//! Generated (sketch) — diff from baseline (see `./minimal.rs`)
//! =============================================================================
//!
//!     impl UserFactory<No> {
//!         pub fn name(self, val: String)
//!             -> Result<UserFactory<Yes>, ValidationError>
//!         { /* body: stores `require_nonempty(val)?` */ }
//!     }

use std::fmt;

use typestate_pipeline::TypestateFactory;

#[derive(Debug)]
struct ValidationError(&'static str);

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
impl std::error::Error for ValidationError {}

#[derive(TypestateFactory)]
#[factory(error = ValidationError)]
#[allow(dead_code)]
struct User {
    #[field(required, setter = require_nonempty, fallible)]
    name: String,
}

fn require_nonempty(value: String) -> Result<String, ValidationError> {
    if value.is_empty() {
        Err(ValidationError("name is empty"))
    } else {
        Ok(value)
    }
}

fn main() {
    // Happy path: `?` unwraps the bag and the chain continues.
    let user = UserFactory::new()
        .name("Alice".to_owned())
        .expect("non-empty")
        .finalize();
    assert_eq!(user.name, "Alice");

    // Failure path: setter returns Err and the bag is consumed.
    assert!(UserFactory::new().name(String::new()).is_err());
}
