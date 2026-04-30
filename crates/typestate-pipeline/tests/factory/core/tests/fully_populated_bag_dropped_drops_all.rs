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
        let _bag = DropTraceFactory::new()
            .primary(Counted::new("p"))
            .secondary(Counted::new("s"));
        assert_eq!(alive(), baseline + 2);
    }
    assert_eq!(alive(), baseline);
}
