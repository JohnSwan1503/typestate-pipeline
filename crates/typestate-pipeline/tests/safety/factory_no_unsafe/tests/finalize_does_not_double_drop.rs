#[path = "shared.rs"]
mod shared;

use shared::{Counted, alive, serialize};
use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
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
        // Both Counted instances are now owned by `drop_trace`. The bag's
        // remaining un-moved fields (just `_markers: PhantomData`) auto-drop
        // trivially — there's no manual Drop impl in safe mode, so the
        // partial move is allowed and well-defined.
        assert_eq!(alive(), baseline + 2);
        let _ = drop_trace;
    }
    assert_eq!(alive(), baseline);
}
