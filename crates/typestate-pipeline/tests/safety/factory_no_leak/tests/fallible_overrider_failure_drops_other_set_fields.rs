#[path = "shared.rs"]
mod shared;

use shared::{Counted, Reject, alive, serialize};
use typestate_pipeline::TypestateFactory;

fn sync_pass_or_reject(val: Counted) -> Result<Counted, Reject> {
    if val.0 == "fail" {
        drop(val);
        Err(Reject)
    } else {
        Ok(val)
    }
}

#[derive(TypestateFactory)]
#[factory(error = Reject)]
struct OverridableBag {
    #[field(required)]
    other: Counted,
    #[field(required, setter = sync_pass_or_reject, fallible, overridable)]
    main: Counted,
}

pub fn main() {
    let _g = serialize();
    let baseline = alive();
    {
        let bag = OverridableBagFactory::new()
            .other(Counted::new("o"))
            .main(Counted::new("m_old"))
            .expect("initial set");
        assert_eq!(alive(), baseline + 2);

        let result = bag.override_main(Counted::new("fail"));
        // `fail` was dropped inside the transformer. `bag` carried both
        // `other` and `m_old`; both should be released by the bag's Drop
        // when the override fails.
        assert!(result.is_err());
        assert_eq!(
            alive(),
            baseline,
            "fallible override failure leaked other set fields",
        );
    }
    assert_eq!(alive(), baseline);
}
