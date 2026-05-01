use super::{Counted, alive, serialize};
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
        let _bag = DropTraceFactory::new();
        // Both slots are `()` here — auto-drop is a no-op for both.
    }
    assert_eq!(alive(), baseline);
}
