//! `#[field(required)]` (the default) emits a bare-named setter
//! (`name(val)`); `#[field(optional)]` emits a `with_<field>` setter
//! (`with_nickname(val)`). The naming change is the only difference —
//! `optional` on its own does **not** relax the finalize bound; the flag
//! still has to be `Yes`. Pair `optional` with `default` (see
//! `./default.rs`) to actually allow finalize without setting it.
//!
//! =============================================================================
//! Generated (sketch) — diff from baseline (see `./minimal.rs`)
//! =============================================================================
//!
//!     impl<F2> UserFactory<No, F2> {
//!         pub fn name(self, val: String)         -> UserFactory<Yes, F2>;
//!     }
//!     impl<F1> UserFactory<F1, No> {
//!         pub fn with_nickname(self, val: String) -> UserFactory<F1, Yes>;
//!     }
//!     // finalize still binds both flags to `Yes`.
//!     impl UserFactory<Yes, Yes> { pub fn finalize(self) -> User; }

use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required)]
    name: String,
    #[field(optional)]
    nickname: String,
}

fn main() {
    let bag = UserFactory::new()
        .name("Alice".to_owned())
        .with_nickname("Al".to_owned());
    let user = bag.finalize();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.nickname, "Al");
}
