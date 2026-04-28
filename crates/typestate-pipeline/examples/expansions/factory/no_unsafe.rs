//! `#[factory(no_unsafe)]` opts an individual derive into the safe-mode
//! codegen path. Storage swaps from `MaybeUninit<T>` to
//! `<Flag as Storage<T>>::Out` (which is `T` for `Yes` and `()` for `No`),
//! so each `(Yes, …)` / `(No, …)` combination is a structurally distinct
//! sister type and the compiler-derived `Drop` handles both shapes —
//! no manual `Drop`, no `unsafe`.
//!
//! Public method names and signatures are **identical** to the
//! `MaybeUninit` path; only the implementation differs. (The `Storage`
//! bound on `finalize`'s impl is the one externally visible signature
//! difference, used by some advanced cases — see `./ready_trait.rs`
//! for the same point on the Ready trait.)
//!
//! Gated on the `no_unsafe` Cargo feature. Without the feature, the
//! attribute is rejected at expansion time so a downstream typo cannot
//! silently cross codegen modes.
//!
//! =============================================================================
//! Generated (sketch) — diff from baseline (see `./minimal.rs`)
//! =============================================================================
//!
//! No public-API diff. The sketch in `./minimal.rs` applies verbatim;
//! `finalize`'s impl uses `<Flag as Storage<T>>::finalize_or` internally
//! instead of an `IS_SET` branch, but that's invisible at the call site.

use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
#[allow(dead_code)]
struct User {
    #[field(required)]
    name: String,
    #[field(default = 18)]
    age: u32,
}

fn main() {
    let user = UserFactory::new().name("Alice".to_owned()).finalize();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.age, 18);

    let user = UserFactory::new()
        .name("Bob".to_owned())
        .with_age(42)
        .finalize();
    assert_eq!(user.age, 42);
}
