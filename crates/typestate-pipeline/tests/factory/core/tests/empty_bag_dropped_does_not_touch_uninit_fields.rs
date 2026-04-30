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
    // A fresh bag has no fields initialized. Dropping it must not
    // call drop on the MaybeUninit slots — Counted's counter should
    // stay where it was.
    let baseline = alive();
    {
        let _bag = DropTraceFactory::new();
        // bag goes out of scope here — flags are all No, so the
        // generated Drop impl skips every field.
    }
    assert_eq!(alive(), baseline, "Drop touched uninit fields");
}
