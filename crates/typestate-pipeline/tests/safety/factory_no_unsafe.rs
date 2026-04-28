//! Safe-mode (`#[factory(no_unsafe)]`) coverage for `#[derive(TypestateFactory)]`.
//!
//! Mirrors the unsafe-mode scenarios from `typestate_factory.rs`,
//! `factory_no_leak.rs`, and `factory_async.rs` but every derive opts into
//! the safe codegen path so the generated bag uses the `Storage<T>`
//! associated-type trick (and therefore zero `MaybeUninit` / `unsafe` /
//! manual `Drop` machinery).
//!
//! Gated on the `no_unsafe` feature: when the feature is off, the
//! `#[factory(no_unsafe)]` attribute is rejected at expansion time so this
//! file would fail to even compile. The corresponding negative trybuild
//! case lives in `tests/ui/factory_no_unsafe_without_feature.rs`.

#![cfg(feature = "no_unsafe")]

use std::{
    future::Future,
    pin::Pin,
    sync::{
        Mutex, MutexGuard,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Waker},
};

use typestate_pipeline::TypestateFactory;

// ---------------------------------------------------------------------------
// Drop bookkeeping — each test asserts ALIVE returns to its baseline so any
// leaked field surfaces as a count mismatch.
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

fn serialize() -> MutexGuard<'static, ()> {
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

// ===========================================================================
// Construction, ordering, getters, default helper
// ===========================================================================

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
struct UserBuilder {
    #[field(required)]
    name: String,
    #[field(required)]
    email: String,
    #[field(default = 18)]
    age: u32,
}

#[test]
fn build_in_order() {
    let user = UserBuilderFactory::new()
        .name("Alice".to_owned())
        .email("alice@example.com".to_owned())
        .with_age(30)
        .finalize();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
    assert_eq!(user.age, 30);
}

#[test]
fn build_in_arbitrary_order() {
    let user = UserBuilderFactory::new()
        .with_age(42)
        .email("bob@example.com".to_owned())
        .name("Bob".to_owned())
        .finalize();
    assert_eq!(user.name, "Bob");
    assert_eq!(user.age, 42);
}

#[test]
fn default_helper_fills_in_optional() {
    let user = UserBuilderFactory::new()
        .name("Carol".to_owned())
        .email("carol@example.com".to_owned())
        .age_default()
        .finalize();
    assert_eq!(user.age, 18);
}

#[test]
fn finalize_uses_default_when_optional_unset() {
    // Optional-with-default field never set → finalize uses the declared
    // default. In safe mode this dispatches via Storage::finalize_or rather
    // than an IS_SET branch.
    let user = UserBuilderFactory::new()
        .name("Dave".to_owned())
        .email("dave@example.com".to_owned())
        .finalize();
    assert_eq!(user.age, 18);
}

#[test]
fn getter_borrows_set_field() {
    let bag = UserBuilderFactory::new()
        .name("Eve".to_owned())
        .email("eve@example.com".to_owned());
    assert_eq!(bag.name(), "Eve");
    assert_eq!(bag.email(), "eve@example.com");
}

// ===========================================================================
// Drop semantics — relies on auto-Drop in safe mode (no manual Drop impl)
// ===========================================================================

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
#[allow(dead_code)]
struct DropTrace {
    #[field(required)]
    primary: Counted,
    #[field(required)]
    secondary: Counted,
}

#[test]
fn empty_bag_dropped_does_not_touch_unset_fields() {
    let _g = serialize();
    let baseline = alive();
    {
        let _bag = DropTraceFactory::new();
        // Both slots are `()` here — auto-drop is a no-op for both.
    }
    assert_eq!(alive(), baseline);
}

#[test]
fn partial_bag_dropped_drops_only_set_fields() {
    let _g = serialize();
    let baseline = alive();
    {
        let _bag = DropTraceFactory::new().primary(Counted::new("p"));
        assert_eq!(alive(), baseline + 1);
        // primary slot is `Counted` (auto-drops), secondary slot is `()`.
    }
    assert_eq!(alive(), baseline);
}

#[test]
fn fully_populated_bag_dropped_drops_all() {
    let _g = serialize();
    let baseline = alive();
    {
        let _bag = DropTraceFactory::new()
            .primary(Counted::new("p"))
            .secondary(Counted::new("s"));
        assert_eq!(alive(), baseline + 2);
    }
    assert_eq!(alive(), baseline);
}

#[test]
fn finalize_does_not_double_drop() {
    let _g = serialize();
    let baseline = alive();
    {
        let bag = DropTraceFactory::new()
            .primary(Counted::new("p"))
            .secondary(Counted::new("s"));
        let drop_trace = bag.finalize();
        // Both Counted instances are now owned by `drop_trace`. The bag's
        // remaining un-moved fields (just `_markers: PhantomData`) auto-drop
        // trivially — there's no manual Drop impl in safe mode, so the
        // partial move is allowed and well-defined.
        assert_eq!(alive(), baseline + 2);
        let _ = drop_trace;
    }
    assert_eq!(alive(), baseline);
}

// ===========================================================================
// Removable — drop_<field> transitions Yes → No, drops the value
// ===========================================================================

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
#[allow(dead_code)]
struct Removable {
    #[field(required, removable)]
    primary: Counted,
    #[field(required)]
    other: Counted,
}

#[test]
fn drop_field_drops_value_once() {
    let _g = serialize();
    let baseline = alive();
    {
        let bag = RemovableFactory::new()
            .primary(Counted::new("p"))
            .other(Counted::new("o"));
        assert_eq!(alive(), baseline + 2);

        let bag = bag.drop_primary();
        assert_eq!(alive(), baseline + 1, "drop_primary should release primary");

        drop(bag);
    }
    assert_eq!(alive(), baseline);
}

#[test]
fn drop_field_then_reset_doesnt_double_drop() {
    let _g = serialize();
    let baseline = alive();
    {
        let bag = RemovableFactory::new()
            .primary(Counted::new("p1"))
            .other(Counted::new("o"))
            .drop_primary()
            .primary(Counted::new("p2"));
        assert_eq!(alive(), baseline + 2);
        drop(bag);
    }
    assert_eq!(alive(), baseline);
}

// ===========================================================================
// Overridable — override_<field> stays in Yes, drops the old value
// ===========================================================================

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
#[allow(dead_code)]
struct Overridable {
    #[field(required, overridable)]
    payload: Counted,
}

#[test]
fn override_drops_old_value() {
    let _g = serialize();
    let baseline = alive();
    {
        let bag = OverridableFactory::new()
            .payload(Counted::new("first"))
            .override_payload(Counted::new("second"));
        // The first Counted was the leftover un-moved slot when we built
        // the new bag literal — auto-drop released it at end-of-scope of
        // the override body. Only the second is alive.
        assert_eq!(alive(), baseline + 1);
        drop(bag);
    }
    assert_eq!(alive(), baseline);
}

// ===========================================================================
// Conditional finalize — optional+default may be Yes OR No at finalize.
// In safe mode this dispatches via Storage::finalize_or.
// ===========================================================================

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
struct Configurable {
    #[field(required)]
    name: String,
    #[field(default = 8)]
    parallelism: u16,
    #[field(default = "https://default.example".to_owned())]
    url: String,
}

#[test]
fn finalize_uses_defaults_when_optional_no() {
    let cfg = ConfigurableFactory::new().name("svc-a".to_owned()).finalize();
    assert_eq!(cfg.name, "svc-a");
    assert_eq!(cfg.parallelism, 8);
    assert_eq!(cfg.url, "https://default.example");
}

#[test]
fn finalize_keeps_explicit_values_when_optional_yes() {
    let cfg = ConfigurableFactory::new()
        .name("svc-b".to_owned())
        .with_parallelism(16)
        .with_url("https://override.example".to_owned())
        .finalize();
    assert_eq!(cfg.parallelism, 16);
    assert_eq!(cfg.url, "https://override.example");
}

#[test]
fn finalize_mixes_set_and_default() {
    let cfg = ConfigurableFactory::new()
        .name("svc-c".to_owned())
        .with_parallelism(4)
        .finalize();
    assert_eq!(cfg.parallelism, 4);
    assert_eq!(cfg.url, "https://default.example");
}

// ===========================================================================
// Custom transformer (sync infallible)
// ===========================================================================

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
struct NormalizedUser {
    #[field(required, setter = trim_name)]
    name: String,
}

fn trim_name(value: String) -> String {
    value.trim().to_owned()
}

#[test]
fn custom_transformer_fn() {
    let u = NormalizedUserFactory::new()
        .name("   Bob   ".to_owned())
        .finalize();
    assert_eq!(u.name, "Bob");
}

// ===========================================================================
// Custom transformer (sync fallible)
// ===========================================================================

#[derive(Debug)]
struct ValidationError(&'static str);

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for ValidationError {}

#[derive(TypestateFactory)]
#[factory(no_unsafe, error = ValidationError)]
struct ValidatedUser {
    #[field(required, setter = require_nonempty, fallible)]
    name: String,
}

fn require_nonempty(value: String) -> Result<String, ValidationError> {
    if value.is_empty() {
        Err(ValidationError("name is empty"))
    } else {
        Ok(value)
    }
}

#[test]
fn fallible_transformer_success() {
    let bag = ValidatedUserFactory::new()
        .name("Carol".to_owned())
        .expect("non-empty");
    assert_eq!(bag.name(), "Carol");
    let u = bag.finalize();
    assert_eq!(u.name, "Carol");
}

#[test]
fn fallible_transformer_failure() {
    match ValidatedUserFactory::new().name(String::new()) {
        Ok(_) => panic!("expected validation failure"),
        Err(ValidationError(msg)) => assert_eq!(msg, "name is empty"),
    }
}

// ===========================================================================
// Leak-safety on fallible setter failure — auto-drop reclaims other fields
// ===========================================================================

#[derive(Debug)]
struct Reject;

impl std::fmt::Display for Reject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("rejected")
    }
}
impl std::error::Error for Reject {}

#[derive(TypestateFactory)]
#[factory(no_unsafe, error = Reject)]
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
        assert_eq!(alive(), baseline + 1);

        let result = bag.main(Counted::new("m"));
        // sync_reject dropped `m`. The `?` short-circuited before consuming
        // `bag` into the new struct literal, so `bag` (with `other`) is
        // dropped intact via the auto-derived Drop.
        assert!(result.is_err());
        assert_eq!(alive(), baseline);
    }
    assert_eq!(alive(), baseline);
}

// ---------------------------------------------------------------------------
// Fallible overrider — old + other should both be released on failure.
// ---------------------------------------------------------------------------

#[derive(TypestateFactory)]
#[factory(no_unsafe, error = Reject)]
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
        // `fail` was dropped inside the transformer; `bag` (with `other`
        // and `m_old`) is dropped intact since `?` short-circuited before
        // we touched `self`.
        assert!(result.is_err());
        assert_eq!(alive(), baseline);
    }
    assert_eq!(alive(), baseline);
}

// ---------------------------------------------------------------------------
// Async setter, future dropped mid-await — bag's `other` should still drop.
// ---------------------------------------------------------------------------

#[derive(TypestateFactory)]
#[factory(no_unsafe, error = Reject)]
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
}

#[test]
fn async_setter_dropped_mid_await_drops_other_set_fields() {
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

// ===========================================================================
// Internal field — set positionally on new(), no setter, getter always works
// ===========================================================================

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
#[allow(dead_code)]
struct WithInternal {
    #[field(internal)]
    namespace: String,
    #[field(required)]
    name: String,
}

#[test]
fn internal_field_round_trips() {
    let bag = WithInternalFactory::new("ns".to_owned());
    // Internal getter is callable on the empty bag.
    assert_eq!(bag.namespace(), "ns");

    let user = bag.name("svc".to_owned()).finalize();
    assert_eq!(user.namespace, "ns");
    assert_eq!(user.name, "svc");
}
