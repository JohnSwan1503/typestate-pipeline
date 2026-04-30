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
            .primary(Counted::new("p"))
            .other(Counted::new("o"));
        assert_eq!(alive(), baseline + 2);

        // drop_primary returns a bag with primary's flag = No; the
        // value was dropped exactly once inside drop_primary.
        let bag = bag.drop_primary();
        assert_eq!(alive(), baseline + 1, "drop_primary should drop primary");

        drop(bag);
    }
    assert_eq!(alive(), baseline);
}
