use super::{Counted, Reject, alive, serialize};
use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(no_unsafe, error = Reject)]
struct OverridableBag {
    #[field(required)]
    other: Counted,
    #[field(required, setter = sync_pass_or_reject, fallible, overridable)]
    main: Counted,
}

fn sync_pass_or_reject(val: Counted) -> Result<Counted, Reject> {
    if val.0 == "fail" {
        drop(val);
        Err(Reject)
    } else {
        Ok(val)
    }
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
        // `fail` was dropped inside the transformer; `bag` (with `other`
        // and `m_old`) is dropped intact since `?` short-circuited before
        // we touched `self`.
        assert!(result.is_err());
        assert_eq!(alive(), baseline);
    }
    assert_eq!(alive(), baseline);
}
