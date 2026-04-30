#[path = "shared.rs"]
mod shared;

use shared::{Counted, Reject, alive, serialize};

use typestate_pipeline::TypestateFactory;

fn sync_reject(val: Counted) -> Result<Counted, Reject> {
    drop(val);
    Err(Reject)
}

#[derive(TypestateFactory)]
#[factory(error = Reject)]
struct SyncFallibleBag {
    #[field(required)]
    other: Counted,
    #[field(required, setter = sync_reject, fallible)]
    main: Counted,
}

pub fn main() {
    let _g = serialize();
    let baseline = alive();
    {
        let bag = SyncFallibleBagFactory::new().other(Counted::new("o"));
        assert_eq!(alive(), baseline + 1, "other was set");

        let result = bag.main(Counted::new("m"));
        // `m` was consumed and dropped by sync_reject. `bag` (carrying `other`)
        // is consumed by the setter; with the leak fix, `?` returns from
        // *outside* the ManuallyDrop scope so `bag` Drops normally and
        // releases `other`.
        assert!(result.is_err());
        assert_eq!(
            alive(),
            baseline,
            "fallible setter failure leaked the other field",
        );
    }
    assert_eq!(alive(), baseline);
}
