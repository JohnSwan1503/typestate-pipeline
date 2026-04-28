//! `#[field(internal)]` makes a field positional on `new(…)` and locked
//! from then on. Four user-visible consequences:
//!
//! - `new(…)` takes the internal field as a parameter.
//! - The bag's flag-generic list does **not** include the internal field
//!   (a struct with two non-internal fields has `Factory<F1, F2>`, not
//!   `Factory<F1, F2, F3>`).
//! - No setter, remover, overrider, or default helper is emitted.
//! - The getter is unconditional — callable on any bag shape.
//!
//! Internal fields are incompatible with `optional`, `default`,
//! `overridable`, `removable`, `setter`, `fallible`, `async_fn`, and
//! `input` — using any of them errors at expansion.
//!
//! =============================================================================
//! Generated (sketch) — diff from baseline (see `./minimal.rs`)
//! =============================================================================
//!
//!     // No flag generic for `namespace`; it lives in the struct as plain T.
//!     struct JobFactory<F1 = No, F2 = No> { /* private */ }
//!
//!     impl JobFactory<No, No> {
//!         pub fn new(namespace: String) -> Self;   // positional
//!     }
//!     impl<F1, F2> JobFactory<F1, F2> {
//!         pub fn namespace(&self) -> &String;     // unconditional getter
//!     }
//!     // (no `fn namespace(self, …)` setter, no `drop_namespace`, no
//!     //  `override_namespace`, no `namespace_default`.)

use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct Job {
    #[field(required, internal)]
    namespace: String,
    #[field(required)]
    parallelism: u16,
    #[field(default = false)]
    verify: bool,
}

fn main() {
    // No `.namespace(...)` chain — the field is positional on `new(...)`.
    let bag = JobFactory::new("eth".to_owned())
        .parallelism(4)
        .with_verify(true);

    // The namespace getter is callable on any bag shape — even before
    // `parallelism` is set:
    let fresh = JobFactory::new("op".to_owned());
    assert_eq!(fresh.namespace(), "op");

    let job = bag.finalize();
    assert_eq!(job.namespace, "eth");
    assert_eq!(job.parallelism, 4);
    assert!(job.verify);
}
