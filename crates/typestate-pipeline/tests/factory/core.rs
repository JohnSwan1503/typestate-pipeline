//! Standalone tests for `#[derive(TypestateFactory)]`.
//!
//! Organized by feature: construction & getters, Drop semantics, removable,
//! overridable, defaults / conditional finalize, custom names, and custom
//! transformers (sync infallible + sync fallible). Pipeline-integrated
//! behavior lives in `factory_pipeline_integration.rs`; async setters and
//! `finalize_async` live in `factory_async.rs`; leak-on-failure regressions
//! live in `factory_no_leak.rs`.

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex, MutexGuard,
};

use typestate_pipeline::TypestateFactory;

// ---------------------------------------------------------------------------
// Drop bookkeeping
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

/// Serialize a leak-tracking test against its peers in this binary so the
/// shared `ALIVE` static can't be raced by another running test. Recovers
/// from poisoning so a previous failure reports independently rather than
/// cascading.
fn serialize() -> MutexGuard<'static, ()> {
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

// ===========================================================================
// Construction, ordering, getters, default helper
// ===========================================================================

#[derive(TypestateFactory)]
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
    let bag = UserBuilderFactory::new()
        .name("Alice".to_owned())
        .email("alice@example.com".to_owned())
        .with_age(30);
    let user = bag.finalize();

    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
    assert_eq!(user.age, 30);
}

#[test]
fn build_in_arbitrary_order() {
    // Setter ordering is irrelevant — the typestate transitions independently.
    let bag = UserBuilderFactory::new()
        .with_age(42)
        .email("bob@example.com".to_owned())
        .name("Bob".to_owned());
    let user = bag.finalize();

    assert_eq!(user.name, "Bob");
    assert_eq!(user.email, "bob@example.com");
    assert_eq!(user.age, 42);
}

#[test]
fn default_helper_fills_in_optional() {
    // `age_default()` is generated because `age` has `#[field(default = 18)]`.
    // It transitions the flag to Yes using the declared default expression.
    let user = UserBuilderFactory::new()
        .name("Carol".to_owned())
        .email("carol@example.com".to_owned())
        .age_default()
        .finalize();

    assert_eq!(user.age, 18);
}

#[test]
fn getter_borrows_set_field() {
    let bag = UserBuilderFactory::new()
        .name("Dave".to_owned())
        .email("dave@example.com".to_owned());

    // Both required-flag fields are now Yes; the getters are available.
    assert_eq!(bag.name(), "Dave");
    assert_eq!(bag.email(), "dave@example.com");

    // Default the optional and finalize.
    let user = bag.age_default().finalize();
    assert_eq!(user.name, "Dave");
}

// ===========================================================================
// Drop semantics — partially-populated and fully-populated bags
// ===========================================================================

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct DropTrace {
    #[field(required)]
    primary: Counted,
    #[field(required)]
    secondary: Counted,
}

#[test]
fn empty_bag_dropped_does_not_touch_uninit_fields() {
    let _g = serialize();
    // A fresh bag has no fields initialized. Dropping it must not call drop
    // on the MaybeUninit slots — Counted's counter should stay where it was.
    let baseline = alive();
    {
        let _bag = DropTraceFactory::new();
        // bag goes out of scope here — flags are all No, so the generated
        // Drop impl skips every field.
    }
    assert_eq!(alive(), baseline, "Drop touched uninit fields");
}

#[test]
fn partial_bag_dropped_drops_only_set_fields() {
    let _g = serialize();
    let baseline = alive();
    {
        let _bag = DropTraceFactory::new().primary(Counted::new("p"));
        assert_eq!(alive(), baseline + 1);
        // bag drops here: primary is Yes (must be dropped), secondary is No
        // (must be skipped). Net: counter returns to baseline.
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
        // Both Counted instances are now owned by `drop_trace`, not the bag.
        // The bag was ManuallyDrop'd during finalize so the values weren't
        // double-dropped.
        assert_eq!(alive(), baseline + 2);
        let _ = drop_trace;
    }
    assert_eq!(alive(), baseline);
}

// ===========================================================================
// `#[field(removable)]` — drop_<field> transitions Yes → No, drops once
// ===========================================================================

#[derive(TypestateFactory)]
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

        // drop_primary returns a bag with primary's flag = No; the value
        // was dropped exactly once inside drop_primary.
        let bag = bag.drop_primary();
        assert_eq!(alive(), baseline + 1, "drop_primary should drop primary");

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
            .primary(Counted::new("p2")); // back to Yes with new value
        assert_eq!(alive(), baseline + 2, "p1 dropped, p2 + o still alive");

        drop(bag);
    }
    assert_eq!(alive(), baseline);
}

// ===========================================================================
// `#[field(overridable)]` — override_<field> stays in Yes, drops the old
// ===========================================================================

#[derive(TypestateFactory)]
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
        // The first Counted has been dropped during override; only the
        // second is alive.
        assert_eq!(alive(), baseline + 1);
        drop(bag);
    }
    assert_eq!(alive(), baseline);
}

// ===========================================================================
// Conditional finalize — optional+default may be Yes OR No at finalize
// ===========================================================================

#[derive(TypestateFactory)]
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
    // Don't set parallelism or url — finalize must still succeed and use
    // the declared defaults.
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
    // Half the optionals set, half defaulted.
    let cfg = ConfigurableFactory::new()
        .name("svc-c".to_owned())
        .with_parallelism(4) // explicit
        // url left at default
        .finalize();
    assert_eq!(cfg.parallelism, 4);
    assert_eq!(cfg.url, "https://default.example");
}

// ===========================================================================
// `#[factory(name = …)]` — custom bag type name
// ===========================================================================

#[derive(TypestateFactory)]
#[factory(name = MyManifestBuilder)]
struct ManifestData {
    #[field(required)]
    title: String,
}

#[test]
fn custom_bag_name() {
    let m = MyManifestBuilder::new()
        .title("dataset-x".to_owned())
        .finalize();
    assert_eq!(m.title, "dataset-x");
}

// ===========================================================================
// `#[field(name = …)]` — custom setter method name
// ===========================================================================

#[derive(TypestateFactory)]
struct LoudUser {
    #[field(required, name = shout_name)]
    name: String,
}

#[test]
fn custom_setter_name() {
    let u = LoudUserFactory::new().shout_name("ALICE".to_owned()).finalize();
    assert_eq!(u.name, "ALICE");
}

// ===========================================================================
// `#[field(setter = fn)]` — custom transformer (sync infallible)
// ===========================================================================

#[derive(TypestateFactory)]
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
// `#[field(setter = fn, fallible)]` — custom transformer (sync fallible)
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
#[factory(error = ValidationError)]
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
