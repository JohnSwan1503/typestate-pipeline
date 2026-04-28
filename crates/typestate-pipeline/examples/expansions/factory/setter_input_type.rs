//! `#[field(input = T)]` (paired with `setter = my_fn`) lets the setter
//! accept a different *input* type than the field stores; the transformer
//! bridges the gap. Convenient when the storage type has wrapping users
//! shouldn't have to spell out (`Option<T>`, `Box<T>`, smart pointers,
//! parsed types).
//!
//! When a `default = …` is also declared, the default helper bypasses the
//! transformer and writes the *field* type directly — the default
//! expression is evaluated as the storage type, not the input type.
//!
//! =============================================================================
//! Generated (sketch) — diff from baseline (see `./minimal.rs`)
//! =============================================================================
//!
//!     impl<F1> ProfileFactory<F1, No> {
//!         // Setter takes `String` (the input type), not `Option<String>`.
//!         pub fn with_worker(self, val: String) -> ProfileFactory<F1, Yes>;
//!         // Default helper writes the field type directly (`Option<String>`),
//!         // bypassing the transformer entirely.
//!         pub fn worker_default(self) -> ProfileFactory<F1, Yes>;
//!     }

use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct Profile {
    #[field(required)]
    name: String,
    /// Storage is `Option<String>`, but the setter takes plain `String`;
    /// `wrap_some` lifts it. The `default = None` expression is typed as
    /// `Option<String>` — the helper bypasses `wrap_some` to write it
    /// directly.
    #[field(default = None, setter = wrap_some, input = String)]
    worker: Option<String>,
}

fn wrap_some(s: String) -> Option<String> {
    Some(s)
}

fn main() {
    // 1) Setter takes `String` — no `Some(...)` at the call site.
    let p = ProfileFactory::new()
        .name("alice".to_owned())
        .with_worker("worker-1".to_owned())
        .finalize();
    assert_eq!(p.worker, Some("worker-1".to_owned()));

    // 2) Default helper writes the field type (`None: Option<String>`).
    let p = ProfileFactory::new()
        .name("bob".to_owned())
        .worker_default()
        .finalize();
    assert_eq!(p.worker, None);

    // 3) Skip the field entirely — finalize evaluates the default.
    let p = ProfileFactory::new().name("carol".to_owned()).finalize();
    assert_eq!(p.worker, None);
}
