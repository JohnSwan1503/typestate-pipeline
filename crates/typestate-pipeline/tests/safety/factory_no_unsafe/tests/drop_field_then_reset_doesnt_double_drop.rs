use super::{Counted, alive, serialize};
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
            .primary(Counted::new("p1"))
            .other(Counted::new("o"))
            .drop_primary()
            .primary(Counted::new("p2"));
        assert_eq!(alive(), baseline + 2);
        drop(bag);
    }
    assert_eq!(alive(), baseline);
}
