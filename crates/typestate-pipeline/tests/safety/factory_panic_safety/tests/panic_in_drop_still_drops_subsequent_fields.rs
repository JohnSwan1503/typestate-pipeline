#[path = "shared.rs"]
mod shared;

use std::panic::{self, AssertUnwindSafe};

use shared::{Counted, PanickyDrop, alive, setup};

use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct DropPanicBag {
    #[field(required)]
    a: PanickyDrop,
    #[field(required)]
    b: Counted,
    #[field(required)]
    c: Counted,
}

pub fn main() {
    let _g = setup();

    let bag = DropPanicBagFactory::new()
        .a(PanickyDrop)
        .b(Counted::new("b"))
        .c(Counted::new("c"));
    assert_eq!(alive(), 2, "b and c are alive");

    let result = panic::catch_unwind(AssertUnwindSafe(|| drop(bag)));
    assert!(result.is_err(), "expected the bag's Drop to panic");
    assert_eq!(
        alive(),
        0,
        "b and c must drop even though a's Drop panicked first",
    );
}
