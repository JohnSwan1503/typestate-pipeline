//! `#[field(default)]` / `#[field(default = expr)]` makes a field optional
//! with a fallback. Two visible changes:
//!
//! 1. A `<field>_default()` helper appears that flips the flag to `Yes`
//!    using the declared default expression.
//! 2. `finalize()` becomes callable whether the field's flag is `Yes`
//!    *or* `No` — when `No`, the default expression is evaluated as the
//!    last step of `finalize`.
//!
//! =============================================================================
//! Generated (sketch) — diff from baseline (see `./minimal.rs`)
//! =============================================================================
//!
//!     impl<F1> UserFactory<F1, No> {
//!         pub fn with_age(self, val: u32) -> UserFactory<F1, Yes>;  // optional → with_
//!         pub fn age_default(self)        -> UserFactory<F1, Yes>;  // default helper
//!     }
//!
//!     // finalize is callable on EITHER state of `age`'s flag:
//!     impl<F2: Satisfiable> UserFactory<Yes, F2> {              // unsafe-mode bound
//!         pub fn finalize(self) -> User;
//!     }
//!     // (safe mode uses `F2: Storage<u32>` instead of `Satisfiable`.)

use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required)]
    name: String,
    #[field(default = 18)]
    age: u32,
}

fn main() {
    // 1) Skip `age` entirely — the declared default fires at finalize().
    let user = UserFactory::new().name("Alice".to_owned()).finalize();
    assert_eq!(user.age, 18);

    // 2) Set `age` explicitly (note `with_` prefix because age is optional).
    let user = UserFactory::new()
        .name("Bob".to_owned())
        .with_age(30)
        .finalize();
    assert_eq!(user.age, 30);

    // 3) Use the helper to flip the flag without specifying the value.
    let user = UserFactory::new()
        .name("Carol".to_owned())
        .age_default()
        .finalize();
    assert_eq!(user.age, 18);
}
