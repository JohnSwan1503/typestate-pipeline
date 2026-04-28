//! Verify `#[field(input = T)]` — setter input type that differs from the
//! stored field type.
//!
//! Use case: `Option<T>` field that the user shouldn't have to wrap in
//! `Some` at every call site. The transformer (`setter = …`) lifts the
//! input value into the storage type.

use typestate_pipeline::TypestateFactory;

#[derive(Debug, TypestateFactory)]
struct Profile {
    #[field(required)]
    name: String,
    /// Worker name — `Option<String>` storage, but the user-facing
    /// setter takes plain `String` and the `wrap_some` transformer lifts.
    #[field(default = None, setter = wrap_some, input = String)]
    worker: Option<String>,
}

fn wrap_some(s: String) -> Option<String> {
    Some(s)
}

#[test]
fn setter_takes_input_type_not_field_type() {
    // The setter accepts a `String`, not an `Option<String>`. The
    // transformer wraps it into the storage shape internally.
    let bag = ProfileFactory::new()
        .name("alice".to_owned())
        .with_worker("worker-1".to_owned());
    let profile = bag.finalize();

    assert_eq!(profile.name, "alice");
    assert_eq!(profile.worker, Some("worker-1".to_owned()));
}

#[test]
fn default_helper_bypasses_transformer() {
    // The `default = None` expression is `Option<String>`, NOT `String`.
    // The default helper must inline a direct field write rather than
    // routing through the setter (which would require `String`). This
    // exercises the bypass path in `gen_default_helper`.
    let bag = ProfileFactory::new().name("bob".to_owned()).worker_default();
    let profile = bag.finalize();

    assert_eq!(profile.worker, None);
}

#[test]
fn unset_default_field_uses_default_at_finalize() {
    // Same end-state as `worker_default()` but reached via the bag's
    // optional-field finalize bound (flag stays `No`, the default expr
    // is evaluated by `finalize`).
    let profile = ProfileFactory::new().name("carol".to_owned()).finalize();
    assert_eq!(profile.worker, None);
}
