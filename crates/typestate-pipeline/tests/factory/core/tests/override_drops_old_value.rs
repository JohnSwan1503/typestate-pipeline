#[path = "shared.rs"]
mod shared;

use shared::{Counted, alive, serialize};
use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct Overridable {
    #[field(required, overridable)]
    payload: Counted,
}

pub fn main() {
    let _g = serialize();
    let baseline = alive();
    {
        let bag = OverridableFactory::new()
            .payload(Counted::new("first"))
            .override_payload(Counted::new("second"));
        // The first Counted has been dropped during override; only
        // the second is alive.
        assert_eq!(alive(), baseline + 1);
        drop(bag);
    }
    assert_eq!(alive(), baseline);
}
