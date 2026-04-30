#[path = "shared.rs"]
mod shared;

use std::panic::{self, AssertUnwindSafe};

use shared::{Counted, PanickyDrop, alive, setup};

use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct RemoveDropPanicBag {
    #[field(required, removable)]
    a: PanickyDrop,
    #[field(required)]
    b: Counted,
    #[field(required)]
    c: Counted,
}

pub fn main() {
    let _g = setup();

    let bag = RemoveDropPanicBagFactory::new()
        .a(PanickyDrop)
        .b(Counted::new("b"))
        .c(Counted::new("c"));
    assert_eq!(alive(), 2);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let _ = bag.drop_a();
    }));
    assert!(result.is_err());
    assert_eq!(
        alive(),
        0,
        "b and c must drop even though the removed `a`'s Drop panicked",
    );
}
