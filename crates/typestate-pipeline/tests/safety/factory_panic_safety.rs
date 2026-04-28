//! Panic-mode regression tests: panicking destructors / panicking
//! `default = …` expressions must not leak the bag's other set fields.
//!
//! These pin the three failure modes the unsafe-mode codegen had before
//! the per-field RAII / read-into-stack-temp restructure:
//!
//! 1. Bag's `Drop` impl: a panic in field N's `T::drop` previously
//!    short-circuited the manual Drop body, leaking fields N+1..end. Now
//!    each field is read out into an owned `Option<T>` stack guard so the
//!    auto-drop's cleanup-on-panic still runs the remaining destructors.
//! 2. `finalize()`'s default-expression branch: a panic in `#default_expr`
//!    previously left fields after it un-`ptr::read`'d inside the
//!    `ManuallyDrop` wrapper, leaking them. Now `finalize()` reads every
//!    initialized field into a stack local *before* evaluating any
//!    default thunk.
//! 3. `override_<field>` / `drop_<field>`: a panic in the OLD value's
//!    `T::drop` (called in-body via `assume_init_drop`) previously leaked
//!    the other fields because the new bag hadn't been built yet. Now the
//!    old value is read into a stack temp and dropped at end-of-scope,
//!    after the new bag is constructed.
//!
//! Each test uses `catch_unwind` to observe the live-counter delta after
//! the unwind settles. `Counted` is a generic alive-counter; `PanickyDrop`
//! panics on its first drop only (a one-shot via `PANICKY_FUSE`) so we can
//! probe the panic path without immediately tripping double-panic abort
//! when an additional `PanickyDrop` is also dropped on unwind.
//!
//! Tests serialize against each other via `LOCK` because the per-binary
//! atomic counters can otherwise race across `cargo test`'s parallel
//! within-binary scheduling.

use std::panic::{self, AssertUnwindSafe};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex, MutexGuard,
};

use typestate_pipeline::TypestateFactory;

// ---------------------------------------------------------------------------
// Drop bookkeeping primitives
// ---------------------------------------------------------------------------

static COUNTED_ALIVE: AtomicUsize = AtomicUsize::new(0);
static PANICKY_FUSE: AtomicUsize = AtomicUsize::new(0);
static LOCK: Mutex<()> = Mutex::new(());

fn alive() -> usize {
    COUNTED_ALIVE.load(Ordering::SeqCst)
}

/// Reset both counters and arm the one-shot panic. Hold the returned
/// guard for the rest of the test to keep it isolated from peers.
fn setup() -> MutexGuard<'static, ()> {
    let g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    COUNTED_ALIVE.store(0, Ordering::SeqCst);
    PANICKY_FUSE.store(1, Ordering::SeqCst);
    g
}

#[derive(Debug)]
#[allow(dead_code)]
struct Counted(&'static str);

impl Counted {
    fn new(label: &'static str) -> Self {
        COUNTED_ALIVE.fetch_add(1, Ordering::SeqCst);
        Counted(label)
    }
}

impl Drop for Counted {
    fn drop(&mut self) {
        COUNTED_ALIVE.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Panics on its first drop and behaves normally thereafter. We need the
/// one-shot because the override / remove tests construct a fresh
/// `PanickyDrop` as the new value, and that one would also drop on unwind
/// — a second panic in `Drop` during unwind would abort the process.
struct PanickyDrop;

impl Drop for PanickyDrop {
    fn drop(&mut self) {
        if PANICKY_FUSE
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1))
            .is_ok()
        {
            panic!("PanickyDrop's Drop panicked (one-shot)");
        }
    }
}

// ===========================================================================
// 1. Bag's Drop: panic in field N's T::drop must not leak fields N+1..end.
// ===========================================================================

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct DropPanicBag {
    #[field(required)]
    a: PanickyDrop,
    #[field(required)]
    b: Counted,
    #[field(required)]
    c: Counted,
}

#[test]
fn panic_in_drop_still_drops_subsequent_fields() {
    let _g = setup();

    let bag = DropPanicBagFactory::new()
        .a(PanickyDrop)
        .b(Counted::new("b"))
        .c(Counted::new("c"));
    assert_eq!(alive(), 2, "b and c are alive");

    let result = panic::catch_unwind(AssertUnwindSafe(|| drop(bag)));
    assert!(result.is_err(), "expected the bag's Drop to panic");
    assert_eq!(
        alive(),
        0,
        "b and c must drop even though a's Drop panicked first",
    );
}

// ===========================================================================
// 2. finalize(): panic in `default = …` must not leak fields after it.
// ===========================================================================

fn panicking_default() -> u32 {
    panic!("default expr panicked");
}

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct FinalizeDefaultPanicBag {
    #[field(required)]
    a: Counted,
    #[field(default = panicking_default())]
    b: u32,
    #[field(required)]
    c: Counted,
}

#[test]
fn panic_in_default_expr_during_finalize_drops_already_read_fields() {
    let _g = setup();

    let bag = FinalizeDefaultPanicBagFactory::new()
        .a(Counted::new("a"))
        // b stays unset → finalize will hit the default branch and panic.
        .c(Counted::new("c"));
    assert_eq!(alive(), 2);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let _ = bag.finalize();
    }));
    assert!(result.is_err(), "default expr should have panicked");
    assert_eq!(
        alive(),
        0,
        "every field that was read into a stack local before the default \
         panicked must drop on unwind",
    );
}

// ===========================================================================
// 3a. override_<field>: panic in OLD value's T::drop must not leak others.
// ===========================================================================

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct OverrideDropPanicBag {
    #[field(required, overridable)]
    a: PanickyDrop,
    #[field(required)]
    b: Counted,
    #[field(required)]
    c: Counted,
}

#[test]
fn panic_in_old_value_drop_during_override_drops_other_fields() {
    let _g = setup();

    let bag = OverrideDropPanicBagFactory::new()
        .a(PanickyDrop)
        .b(Counted::new("b"))
        .c(Counted::new("c"));
    assert_eq!(alive(), 2);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        // OLD `a`'s Drop panics. The new `a` (also a fresh PanickyDrop, but
        // the fuse is one-shot so its Drop is a no-op) lives in the
        // already-built new bag, which the unwind path drops via the
        // bag's panic-safe Drop.
        let _ = bag.override_a(PanickyDrop);
    }));
    assert!(result.is_err());
    assert_eq!(
        alive(),
        0,
        "b and c must drop even though the old `a`'s Drop panicked",
    );
}

// ===========================================================================
// 3b. drop_<field>: panic in OLD value's T::drop must not leak others.
// ===========================================================================

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct RemoveDropPanicBag {
    #[field(required, removable)]
    a: PanickyDrop,
    #[field(required)]
    b: Counted,
    #[field(required)]
    c: Counted,
}

#[test]
fn panic_in_old_value_drop_during_remove_drops_other_fields() {
    let _g = setup();

    let bag = RemoveDropPanicBagFactory::new()
        .a(PanickyDrop)
        .b(Counted::new("b"))
        .c(Counted::new("c"));
    assert_eq!(alive(), 2);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let _ = bag.drop_a();
    }));
    assert!(result.is_err());
    assert_eq!(
        alive(),
        0,
        "b and c must drop even though the removed `a`'s Drop panicked",
    );
}
