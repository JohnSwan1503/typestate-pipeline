#[path = "shared.rs"]
mod shared;

use shared::{Counted, alive, serialize};
use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct Removable {
    #[field(required, removable)]
    primary: Counted,
    #[field(required)]
    other: Counted,
}

pub fn main() {
    let _g = serialize();
    let baseline = alive();
    {
        let bag = RemovableFactory::new()
            .primary(Counted::new("p1"))
            .other(Counted::new("o"))
            .drop_primary()
            .primary(Counted::new("p2")); // back to Yes with new value
        assert_eq!(alive(), baseline + 2, "p1 dropped, p2 + o still alive");

        drop(bag);
    }
    assert_eq!(alive(), baseline);
}
