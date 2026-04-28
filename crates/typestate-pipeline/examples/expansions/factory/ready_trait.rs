//! Every `#[derive(TypestateFactory)]` also emits a companion trait
//! `<BagName>Ready` that auto-impls on every flag combination matching
//! `finalize()`'s bounds. Use it to write generic code over "any
//! finalize-callable bag" without spelling out the flag tuple.
//!
//! The trait method is `into_finalized` (not `finalize`) — using the same
//! name as the inherent would make the auto-impl body recurse instead of
//! delegating to the inherent.
//!
//! =============================================================================
//! Generated (sketch) — addition to baseline (see `./minimal.rs`)
//! =============================================================================
//!
//!     trait UserFactoryReady: Sized {
//!         fn into_finalized(self) -> User;
//!     }
//!     impl<F1: Satisfied, F2: Satisfiable> UserFactoryReady       // unsafe-mode
//!         for UserFactory<F1, F2> { /* delegates to finalize() */ }
//!     // (safe mode: `F1` is pinned to concrete `Yes`, `F2: Storage<u32>`.)

use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required)]
    name: String,
    #[field(default = 18)]
    age: u32,
}

/// Generic over any finalize-callable shape — a stand-in for a downstream
/// API that wants to accept "any built bag" without spelling out the flag
/// tuple.
fn finalize_anything<B: UserFactoryReady>(bag: B) -> User {
    bag.into_finalized()
}

fn main() {
    // Required `name` set, optional `age` left to default — Ready.
    let bag = UserFactory::new().name("Alice".to_owned());
    let user = finalize_anything(bag);
    assert_eq!(user.age, 18);

    // Same trait dispatch works when the optional was set explicitly.
    let bag = UserFactory::new().name("Bob".to_owned()).with_age(42);
    let user = finalize_anything(bag);
    assert_eq!(user.age, 42);
}
