#[path = "shared.rs"]
mod shared;

use shared::{Counted, Reject, alive, serialize};
use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(no_unsafe, error = Reject)]
struct SyncFallibleBag {
    #[field(required)]
    other: Counted,
    #[field(required, setter = sync_reject, fallible)]
    main: Counted,
}

fn sync_reject(val: Counted) -> Result<Counted, Reject> {
    drop(val);
    Err(Reject)
}

pub fn main() {
    let _g = serialize();
    let baseline = alive();
    {
        let bag = SyncFallibleBagFactory::new().other(Counted::new("o"));
        assert_eq!(alive(), baseline + 1);

        let result = bag.main(Counted::new("m"));
        // sync_reject dropped `m`. The `?` short-circuited before consuming
        // `bag` into the new struct literal, so `bag` (with `other`) is
        // dropped intact via the auto-derived `Drop`.
        assert!(result.is_err());
        assert_eq!(alive(), baseline);
    }
    assert_eq!(alive(), baseline);
}
