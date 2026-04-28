//! Verify the auto-generated `<BagName>Ready` trait — the where-clause
//! shorthand for "this flag tuple is finalize-callable".
//!
//! The trait is impl'd on every flag combination matching `finalize`'s
//! bounds (required = `Satisfied`, optional-with-default = `Satisfiable`).
//! Partially-set bags must be rejected by the type system.

use typestate_pipeline::TypestateFactory;

#[derive(Debug, TypestateFactory)]
struct Profile {
    #[field(required)]
    name: String,
    #[field(required)]
    handle: String,
    #[field(default = 18)]
    age: u32,
}

#[test]
fn ready_trait_is_implemented_when_required_flags_yes() {
    let bag = ProfileFactory::new()
        .name("alice".to_owned())
        .handle("@alice".to_owned());
    // `age` flag stays `No` — bag should still be `Ready` because `age`
    // is optional-with-default.
    let profile = generic_finalize(bag);
    assert_eq!(profile.name, "alice");
    assert_eq!(profile.handle, "@alice");
    assert_eq!(profile.age, 18);
}

#[test]
fn ready_trait_works_when_optional_set_too() {
    let bag = ProfileFactory::new()
        .name("bob".to_owned())
        .handle("@bob".to_owned())
        .with_age(42);
    let profile = generic_finalize(bag);
    assert_eq!(profile.age, 42);
}

/// Generic over any `ProfileFactoryReady` bag — a stand-in for a downstream user's
/// `where B: ProfileFactoryReady` bound. Compiling this fn at all is the witness
/// that the macro emitted the trait + impl correctly.
fn generic_finalize<B: ProfileFactoryReady>(bag: B) -> Profile {
    bag.into_finalized()
}

// Compile-time negative assertion: a bag with a required field unset MUST
// NOT implement `ProfileFactoryReady`. We can't directly write a `!T: Trait`
// bound, but we can witness this by attempting to call `generic_finalize`
// on a partial bag in a UI test; here we just lock in the positive
// surface. The companion UI test
// `ready_trait_rejects_unset_required.rs` covers the negative side.

#[test]
fn dispatch_via_trait_matches_inherent_finalize() {
    // The trait method must produce the exact same value as the inherent
    // — it just delegates. This catches regressions where the trait impl
    // body diverges from the inherent's body (e.g. forgetting to apply
    // optional-with-default branching).
    let trait_path = ProfileFactory::new()
        .name("c".to_owned())
        .handle("@c".to_owned())
        .with_age(7);
    let inherent_path = ProfileFactory::new()
        .name("c".to_owned())
        .handle("@c".to_owned())
        .with_age(7);

    let via_trait = ProfileFactoryReady::into_finalized(trait_path);
    let via_inherent = inherent_path.finalize();

    assert_eq!(via_trait.name, via_inherent.name);
    assert_eq!(via_trait.handle, via_inherent.handle);
    assert_eq!(via_trait.age, via_inherent.age);
}
