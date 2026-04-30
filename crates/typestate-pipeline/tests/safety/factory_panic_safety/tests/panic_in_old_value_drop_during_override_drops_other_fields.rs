#[path = "shared.rs"]
mod shared;

use std::panic::{self, AssertUnwindSafe};

use shared::{Counted, PanickyDrop, alive, setup};

use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct OverrideDropPanicBag {
    #[field(required, overridable)]
    a: PanickyDrop,
    #[field(required)]
    b: Counted,
    #[field(required)]
    c: Counted,
}

pub fn main() {
    let _g = setup();

    let bag = OverrideDropPanicBagFactory::new()
        .a(PanickyDrop)
        .b(Counted::new("b"))
        .c(Counted::new("c"));
    assert_eq!(alive(), 2);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        // OLD `a`'s Drop panics. The new `a` (a fresh `PanickyDrop`, but the
        // fuse is one-shot so its Drop is a no-op) lives in the already-built
        // new bag, which the unwind path drops via the bag's panic-safe Drop.
        let _ = bag.override_a(PanickyDrop);
    }));
    assert!(result.is_err());
    assert_eq!(
        alive(),
        0,
        "b and c must drop even though the old `a`'s Drop panicked",
    );
}
