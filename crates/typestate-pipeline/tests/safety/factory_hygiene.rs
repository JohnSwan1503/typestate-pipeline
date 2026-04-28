//! Hygiene regression tests.
//!
//! Pins the rename of macro-internal bindings to a `__tsh_` prefix so a
//! user struct's field names — including names that the previous
//! generated code used internally (`this`, `_markers`, `__field_value`,
//! `__old_field`, `__new_bag`) — no longer collide with the bag.
//!
//! Each `#[derive(TypestateFactory)]` here would have failed to compile
//! under the old codegen because the macro emitted a `_markers:
//! PhantomData<…>` field next to a user-declared `_markers`, or
//! introduced a `let this = …` that the user's `default = …` expression
//! could shadow.
//!
//! These tests are pure compile-passes; the runtime assertions just
//! confirm the round-trip is intact.

use typestate_pipeline::TypestateFactory;

// ---------------------------------------------------------------------------
// Field names that match macro-internal bindings under the old codegen.
// ---------------------------------------------------------------------------

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct CollidingFieldNames {
    /// Previously collided with `_markers: PhantomData<…>` on the bag.
    #[field(required)]
    _markers: u32,
    /// Same family — would have shadowed the macro-internal `this`
    /// binding if a `default = …` expression referenced `this.this`.
    #[field(required)]
    this: u32,
    /// Same family — would have shadowed `__field_value`.
    #[field(required)]
    __field_value: u32,
    /// And the override / remove temps.
    #[field(required, overridable, removable)]
    __old_field: u32,
    #[field(required, overridable, removable)]
    __new_bag: u32,
}

#[test]
fn struct_with_field_names_matching_macro_internals_compiles() {
    let s = CollidingFieldNamesFactory::new()
        ._markers(1)
        .this(2)
        .__field_value(3)
        .__old_field(4)
        .__new_bag(5)
        .finalize();
    assert_eq!(s._markers, 1);
    assert_eq!(s.this, 2);
    assert_eq!(s.__field_value, 3);
    assert_eq!(s.__old_field, 4);
    assert_eq!(s.__new_bag, 5);
}

#[test]
fn override_and_drop_field_named_old_field_round_trips() {
    let s = CollidingFieldNamesFactory::new()
        ._markers(1)
        .this(2)
        .__field_value(3)
        .__old_field(4)
        .__new_bag(5)
        .override___old_field(40)
        .drop___new_bag()
        .__new_bag(50)
        .finalize();
    assert_eq!(s.__old_field, 40);
    assert_eq!(s.__new_bag, 50);
}

// ---------------------------------------------------------------------------
// `default = …` expression with user-declared bindings of its own.
// ---------------------------------------------------------------------------

fn user_helper() -> u32 {
    42
}

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct DefaultUsesUserScope {
    #[field(required)]
    name: String,
    /// The default expression resolves a free function in the user's
    /// scope. Hygiene fix must not block this.
    #[field(default = user_helper())]
    answer: u32,
}

#[test]
fn default_expression_can_call_user_scope_function() {
    let s = DefaultUsesUserScopeFactory::new()
        .name("Alice".to_owned())
        .finalize();
    assert_eq!(s.answer, 42);
}
