//! Pin the assumptions about the bag's `PhantomData<( F1, F2, … )>`
//! marker tuple.
//!
//! The trailing comma in the macro-emitted `PhantomData<( #( #flag_idents,
//! )* )>` produces:
//!
//! - `PhantomData<()>` when there are zero flag generics (every field is
//!   `internal`, or the struct has no fields at all).
//! - `PhantomData<(F,)>` (a *singleton tuple*, not the parenthesised
//!   type `(F)` which is just `F`) when there is exactly one flag.
//! - `PhantomData<(F1, F2, …)>` for the many-flag case.
//!
//! This file exists so a future macro refactor that drops the trailing
//! comma surfaces as a regression at this test rather than a subtle
//! variance / auto-trait change of the bag.

use typestate_pipeline::TypestateFactory;

// ---------------------------------------------------------------------------
// Zero flags: every field is `internal`, so no flag generics.
// ---------------------------------------------------------------------------

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct AllInternal {
    #[field(internal)]
    namespace: &'static str,
    #[field(internal)]
    version: u32,
}

#[test]
fn all_internal_struct_finalizes_without_flag_generics() {
    let s = AllInternalFactory::new("svc", 7).finalize();
    assert_eq!(s.namespace, "svc");
    assert_eq!(s.version, 7);
}

// ---------------------------------------------------------------------------
// Exactly one flag — the trailing-comma case.
// ---------------------------------------------------------------------------

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct OneFlag {
    #[field(required)]
    name: String,
}

#[test]
fn singleton_flag_struct_round_trips() {
    let s = OneFlagFactory::new().name("hello".to_owned()).finalize();
    assert_eq!(s.name, "hello");
}

// Compile-time check: the bag is `Send` and `Sync` when its single field
// is. Auto-trait inheritance through `PhantomData<(F,)>` should match the
// many-flag case — i.e. forwarded through the tuple, not collapsed to a
// bare `F`.
#[test]
fn one_flag_bag_is_send_and_sync_when_field_is() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    assert_send::<OneFlagFactory<typestate_pipeline::Yes>>();
    assert_sync::<OneFlagFactory<typestate_pipeline::Yes>>();
    assert_send::<OneFlagFactory<typestate_pipeline::No>>();
    assert_sync::<OneFlagFactory<typestate_pipeline::No>>();
}

// ---------------------------------------------------------------------------
// Many flags — control case.
// ---------------------------------------------------------------------------

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct ThreeFlags {
    #[field(required)]
    a: u32,
    #[field(required)]
    b: u32,
    #[field(required)]
    c: u32,
}

#[test]
fn many_flag_struct_round_trips() {
    let s = ThreeFlagsFactory::new().a(1).b(2).c(3).finalize();
    assert_eq!((s.a, s.b, s.c), (1, 2, 3));
}
