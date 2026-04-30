#[path = "shared.rs"]
mod shared;

use shared::{Counted, alive, serialize};
use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(error = Reject)]
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

#[derive(Default)]
struct PendOnce {
    polled: bool,
}

impl Future for PendOnce {
    type Output = ();
    fn poll(
        mut self: core::pin::Pin<&mut Self>,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<()> {
        if self.polled {
            core::task::Poll::Ready(())
        } else {
            self.polled = true;
            core::task::Poll::Pending
        }
    }
}

fn poll_once<F: Future>(fut: F) {
    let waker = core::task::Waker::noop();
    let mut cx = core::task::Context::from_waker(waker);
    let mut fut = Box::pin(fut);
    let _ = fut.as_mut().poll(&mut cx);
    // fut drops here, regardless of completion.
}

pub fn main() {
    let _g = serialize();
    let baseline = alive();
    {
        let bag = AsyncSetterBagFactory::new().other(Counted::new("o"));
        assert_eq!(alive(), baseline + 1);

        // The async setter captures `bag` (owns `other`) and `m` (its `val`
        // arg) into the returned future. Polling suspends at PendOnce; the
        // bag's `other` should be released when we drop the future without
        // resuming.
        let fut = bag.main(Counted::new("m"));
        assert_eq!(alive(), baseline + 2, "m moved into the future");
        poll_once(fut);
        assert_eq!(
            alive(),
            baseline,
            "dropping a suspended async setter leaked the other field",
        );
    }
    assert_eq!(alive(), baseline);
}
