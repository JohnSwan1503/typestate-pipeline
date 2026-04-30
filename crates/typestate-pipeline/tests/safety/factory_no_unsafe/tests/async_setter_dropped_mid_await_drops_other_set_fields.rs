#[path = "shared.rs"]
mod shared;

use shared::{Counted, PendOnce, Reject, alive, poll_once, serialize};
use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(no_unsafe, error = Reject)]
struct AsyncSetterBag {
    #[field(required)]
    other: Counted,
    #[field(required, setter = async_pending, async_fn)]
    main: Counted,
}

async fn async_pending(val: Counted) -> Counted {
    PendOnce::default().await;
    val
}

pub fn main() {
    let _g = serialize();
    let baseline = alive();
    {
        let bag = AsyncSetterBagFactory::new().other(Counted::new("o"));
        assert_eq!(alive(), baseline + 1);

        let fut = bag.main(Counted::new("m"));
        assert_eq!(alive(), baseline + 2, "m moved into the future");
        poll_once(fut);
        assert_eq!(alive(), baseline);
    }
    assert_eq!(alive(), baseline);
}
