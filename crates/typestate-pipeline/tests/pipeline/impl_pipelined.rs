//! Smoke tests for [`impl_pipelined!`] — verifies the macro emits a usable
//! [`Pipelined`] impl and [`IntoFuture`] forwarding for the conventional
//! carrier shape.
//!
//! End-to-end exercise of the trait against `#[transitions]` (with `error`
//! omitted) lives in `transitions_via_pipelined.rs`.

use std::fmt;

use typestate_pipeline::{
    impl_pipelined, BoxFuture, InFlight, Mode, Pipeline, Pipelined, Resolved,
};

#[derive(Debug)]
struct Client;

#[derive(Debug)]
struct DummyError;

impl fmt::Display for DummyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("dummy")
    }
}
impl std::error::Error for DummyError {}

struct Author<'a, S, M = Resolved>(Pipeline<'a, Client, (), S, DummyError, M>)
where
    M: Mode<'a, S, DummyError>;

impl_pipelined!(Author, ctx = Client, error = DummyError);

#[derive(Debug, Clone)]
struct Started;

#[derive(Debug, Clone)]
struct Finished;

#[test]
fn pipelined_associated_types_resolve() {
    fn assert_pipelined<'a, T>()
    where
        T: Pipelined<'a, Ctx = Client, Error = DummyError, Tag = ()>,
    {
    }
    assert_pipelined::<Author<'_, Started, Resolved>>();
    assert_pipelined::<Author<'_, Started, InFlight>>();
}

#[test]
fn gat_projections_are_correct() {
    // Resolved<NS> projects to NS-stated Resolved-mode carrier.
    fn assert_resolved_projection<'a, A>()
    where
        A: Pipelined<'a>,
        A::Resolved<Finished>: Pipelined<'a, State = Finished, Mode = Resolved>,
    {
    }
    assert_resolved_projection::<Author<'_, Started, Resolved>>();

    // InFlight<NS> projects similarly.
    fn assert_inflight_projection<'a, A>()
    where
        A: Pipelined<'a>,
        A::Ctx: Sync,
        A::Error: Send,
        A::InFlight<Finished>: Pipelined<'a, State = Finished, Mode = InFlight>,
    {
    }
    assert_inflight_projection::<Author<'_, Started, Resolved>>();
}

#[tokio::test]
async fn intofuture_drives_inflight_back_to_resolved() {
    let client = Client;
    let pending: BoxFuture<'_, Result<Started, DummyError>> = Box::pin(async { Ok(Started) });
    let in_flight = Author(Pipeline::in_flight(&client, pending));
    let resolved: Author<Started, Resolved> = in_flight.await.unwrap();
    let _ = resolved; // type-check only.
}

// Sanity: customizing the tag also compiles.
struct Tagged<'a, S, M = Resolved>(Pipeline<'a, Client, MyTag, S, DummyError, M>)
where
    M: Mode<'a, S, DummyError>;

struct MyTag;

impl_pipelined!(Tagged, ctx = Client, error = DummyError, tag = MyTag);

#[test]
fn tagged_pipelined_resolves() {
    fn assert<'a, T>()
    where
        T: Pipelined<'a, Tag = MyTag>,
    {
    }
    assert::<Tagged<'_, Started, Resolved>>();
}
