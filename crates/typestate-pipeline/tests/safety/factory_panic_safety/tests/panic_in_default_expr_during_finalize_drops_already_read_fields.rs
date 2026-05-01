use std::panic::{self, AssertUnwindSafe};

use super::{Counted, alive, setup};

use typestate_pipeline::TypestateFactory;

fn panicking_default() -> u32 {
    panic!("default expr panicked");
}

#[derive(TypestateFactory)]
struct FinalizeDefaultPanicBag {
    #[field(required)]
    a: Counted,
    #[field(default = panicking_default())]
    b: u32,
    #[field(required)]
    c: Counted,
}

pub fn main() {
    let _g = setup();

    let bag = FinalizeDefaultPanicBagFactory::new()
        .a(Counted::new("a"))
        // `b` stays unset → finalize hits the default branch and panics.
        .c(Counted::new("c"));
    assert_eq!(alive(), 2);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let _ = bag.finalize();
    }));
    assert!(result.is_err(), "default expr should have panicked");
    assert_eq!(
        alive(),
        0,
        "every field that was read into a stack local before the default \
         panicked must drop on unwind",
    );
}
