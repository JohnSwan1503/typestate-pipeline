use super::{Counted, alive, serialize};
use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
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
        // The first Counted was the leftover un-moved slot when we built
        // the new bag literal — auto-drop released it at end-of-scope of
        // the override body. Only the second is alive.
        assert_eq!(alive(), baseline + 1);
        drop(bag);
    }
    assert_eq!(alive(), baseline);
}
