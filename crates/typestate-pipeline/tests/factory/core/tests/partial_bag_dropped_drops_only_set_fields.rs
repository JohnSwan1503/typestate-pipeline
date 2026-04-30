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
        let _bag = DropTraceFactory::new().primary(Counted::new("p"));
        assert_eq!(alive(), baseline + 1);
        // bag drops here: primary is Yes (must be dropped),
        // secondary is No (must be skipped). Net: counter returns to
        // baseline.
    }
    assert_eq!(alive(), baseline);
}
