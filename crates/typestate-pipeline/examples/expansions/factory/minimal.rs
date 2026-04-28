//! Baseline `#[derive(TypestateFactory)]` — every field `required`, no
//! options. The four pieces a derived factory always emits:
//! `<Name>Factory::new()`, one setter per field that flips the matching
//! flag `No -> Yes`, getters that become callable once the flag is `Yes`,
//! and `finalize()` consuming the bag back into the original struct.
//!
//! =============================================================================
//! Generated (sketch)
//! =============================================================================
//!
//!     struct UserFactory<F1 = No, F2 = No> { /* private */ }
//!
//!     impl UserFactory<No, No> {
//!         pub fn new() -> Self;
//!     }
//!     impl Default for UserFactory<No, No> { /* delegates to new() */ }
//!
//!     // Setters: applicable while the matching flag is `No`. Consume `self`,
//!     // flip the flag to `Yes`. The other flag is left as a free generic.
//!     impl<F2> UserFactory<No, F2> {
//!         pub fn name(self, val: String) -> UserFactory<Yes, F2>;
//!     }
//!     impl<F1> UserFactory<F1, No> {
//!         pub fn  age(self, val: u32)    -> UserFactory<F1, Yes>;
//!     }
//!
//!     // Getters: applicable while the matching flag is `Yes`.
//!     impl<F2> UserFactory<Yes, F2> { pub fn name(&self) -> &String; }
//!     impl<F1> UserFactory<F1, Yes> { pub fn  age(&self) -> &u32;    }
//!
//!     // Finalize: callable once every required flag is `Yes`.
//!     impl UserFactory<Yes, Yes> {
//!         pub fn finalize(self) -> User;
//!     }
//!
//!     // Companion "this bag is finalize-callable" trait. Lets downstream
//!     // code write `where B: UserFactoryReady` instead of spelling out the
//!     // flag tuple. See `./ready_trait.rs`.
//!     trait UserFactoryReady: Sized {
//!         fn into_finalized(self) -> User;
//!     }
//!     impl<F1: Satisfied, F2: Satisfied> UserFactoryReady
//!         for UserFactory<F1, F2> { /* delegates to finalize() */ }

use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct User {
    #[field(required)]
    name: String,
    #[field(required)]
    age: u32,
}

fn main() {
    let bag = UserFactory::new().name("Alice".to_owned()).age(30);

    // Getters resolve once the matching flag is `Yes`.
    assert_eq!(bag.name(), "Alice");
    assert_eq!(*bag.age(), 30);

    let user = bag.finalize();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.age, 30);
}
