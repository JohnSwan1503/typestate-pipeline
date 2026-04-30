#[path = "shared.rs"]
mod shared;

use shared::{Counted, alive, serialize};
use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct DropTrace {
    #[field(required)]
    primary: Counted,
    #[field(required)]
    secondary: Counted,
}

pub fn main() {
    let _g = serialize();
    let baseline = alive();
    {
        let bag = DropTraceFactory::new()
            .primary(Counted::new("p"))
            .secondary(Counted::new("s"));
        let drop_trace = bag.finalize();
        // Both Counted instances are now owned by `drop_trace`, not
        // the bag. The bag was ManuallyDrop'd during finalize so the
        // values weren't double-dropped.
        assert_eq!(alive(), baseline + 2);
        let _ = drop_trace;
    }
    assert_eq!(alive(), baseline);
}
