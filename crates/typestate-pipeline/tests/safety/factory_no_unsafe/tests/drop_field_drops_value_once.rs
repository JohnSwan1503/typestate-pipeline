#[path = "shared.rs"]
mod shared;

use shared::{Counted, alive, serialize};
use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
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

        let bag = bag.drop_primary();
        assert_eq!(alive(), baseline + 1, "drop_primary should release primary");

        drop(bag);
    }
    assert_eq!(alive(), baseline);
}
