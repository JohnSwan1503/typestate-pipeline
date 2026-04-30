use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

/// One-shot pending future: `Pending` on first poll, `Ready` on second.
/// Used by the async-setter-cancellation test to suspend the future
/// deterministically without a runtime.
#[derive(Default)]
pub struct PendOnce {
    polled: bool,
}

impl Future for PendOnce {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.polled {
            Poll::Ready(())
        } else {
            self.polled = true;
            Poll::Pending
        }
    }
}

/// Poll a future once with a no-op waker, then drop it. Lets a test
/// observe the partial-drop path without wiring up tokio.
pub fn poll_once<F: Future>(fut: F) {
    let waker = Waker::noop();
    let mut cx = Context::from_waker(waker);
    let mut fut = Box::pin(fut);
    let _ = fut.as_mut().poll(&mut cx);
}
