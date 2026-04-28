//! Regression tests: a failing fallible/async transformer must not leak the
//! other set fields on the input bag.
//!
//! The bag stores fields in `MaybeUninit`, and setters/overriders consume
//! `self` via `ManuallyDrop` so they can `ptr::read` the unchanged fields
//! into the new bag. If the transformer is evaluated *inside* that
//! `ManuallyDrop` scope and short-circuits with `?` (or its enclosing future
//! is dropped mid-await), the original bag's set fields leak — `ManuallyDrop`
//! suppresses Drop and the partial struct literal's `MaybeUninit` temporaries
//! suppress it too. The transformer must run *before* the `ManuallyDrop` step.
//!
//! Each test serializes against the others via `LOCK` because `ALIVE` is a
//! per-binary atomic — `cargo test` runs tests within a binary in parallel by
//! default, and an unrelated test creating `Counted`s would race the
//! `assert_eq!(alive(), baseline)` check.

use std::{
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex, MutexGuard,
    },
    task::{Context, Poll, Waker},
};

use typestate_pipeline::TypestateFactory;

// ---------------------------------------------------------------------------
// Drop-bookkeeping primitives
// ---------------------------------------------------------------------------

static ALIVE: AtomicUsize = AtomicUsize::new(0);
static LOCK: Mutex<()> = Mutex::new(());

#[derive(Debug)]
#[allow(dead_code)]
struct Counted(&'static str);

impl Counted {
    fn new(label: &'static str) -> Self {
        ALIVE.fetch_add(1, Ordering::SeqCst);
        Counted(label)
    }
}

impl Drop for Counted {
    fn drop(&mut self) {
        ALIVE.fetch_sub(1, Ordering::SeqCst);
    }
}

fn alive() -> usize {
    ALIVE.load(Ordering::SeqCst)
}

/// Serialize a test against its peers in this file. Recovers from poisoning
/// so a previous failing test doesn't cascade — each test must report its
/// own leak independently.
fn serialize() -> MutexGuard<'static, ()> {
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

#[derive(Debug)]
struct Reject;

impl std::fmt::Display for Reject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("rejected")
    }
}
impl std::error::Error for Reject {}

// ---------------------------------------------------------------------------
// Sync fallible setter — the simplest manifestation of the bug.
// ---------------------------------------------------------------------------

#[derive(TypestateFactory)]
#[factory(error = Reject)]
#[allow(dead_code)]
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

#[test]
fn fallible_setter_failure_drops_other_set_fields() {
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

// ---------------------------------------------------------------------------
// Sync fallible overrider — worst case: pre-fix, the OLD value was already
// `assume_init_drop`-ed before the transformer ran, so failure both leaked
// other fields *and* destroyed the old field with no replacement.
// ---------------------------------------------------------------------------

#[derive(TypestateFactory)]
#[factory(error = Reject)]
#[allow(dead_code)]
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

#[test]
fn fallible_overrider_failure_drops_other_set_fields() {
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

// ---------------------------------------------------------------------------
// Async setter, future dropped mid-await — the bug also bites without an
// explicit `?`. We hand-poll once to suspend at the inner await, then drop
// the future. With the fix, `self` was never moved into `ManuallyDrop` so
// dropping the future drops `self` normally.
// ---------------------------------------------------------------------------

#[derive(TypestateFactory)]
#[factory(error = Reject)]
#[allow(dead_code)]
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
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.polled {
            Poll::Ready(())
        } else {
            self.polled = true;
            Poll::Pending
        }
    }
}

fn poll_once<F: Future>(fut: F) {
    let waker = Waker::noop();
    let mut cx = Context::from_waker(waker);
    let mut fut = Box::pin(fut);
    let _ = fut.as_mut().poll(&mut cx);
    // fut drops here, regardless of completion.
}

#[test]
fn async_setter_dropped_mid_await_drops_other_set_fields() {
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
