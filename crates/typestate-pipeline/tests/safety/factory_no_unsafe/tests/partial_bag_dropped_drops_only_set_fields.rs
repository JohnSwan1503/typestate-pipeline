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
        let _bag = DropTraceFactory::new().primary(Counted::new("p"));
        assert_eq!(alive(), baseline + 1);
        // primary slot is `Counted` (auto-drops), secondary slot is `()`.
    }
    assert_eq!(alive(), baseline);
}
