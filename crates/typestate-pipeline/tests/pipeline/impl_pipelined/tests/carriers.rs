use typestate_pipeline::{BoxFuture, InFlight, Mode, Pipeline, Resolved, impl_pipelined};

use super::error::DummyError;
use super::state_types::{MyTag, Started};

#[derive(Debug)]
pub struct Client;

// Hand-rolled carrier — `impl_pipelined!` fills in the trait
// plumbing without declaring the struct. Pub field so per-test files
// can construct directly.
pub struct Author<'a, S, M = Resolved>(pub Pipeline<'a, Client, (), S, DummyError, M>)
where
    M: Mode<'a, S, DummyError>;

impl_pipelined!(Author, ctx = Client, error = DummyError);

// Tagged carrier — sanity check that customizing the tag also
// compiles.
pub struct Tagged<'a, S, M = Resolved>(pub Pipeline<'a, Client, MyTag, S, DummyError, M>)
where
    M: Mode<'a, S, DummyError>;

impl_pipelined!(Tagged, ctx = Client, error = DummyError, tag = MyTag);

// ---------------------------------------------------------------------------
// Helper — wrap the Author InFlight construction for the IntoFuture
// test.
// ---------------------------------------------------------------------------

pub fn started_inflight<'a>(
    client: &'a Client,
    pending: BoxFuture<'a, Result<Started, DummyError>>,
) -> Author<'a, Started, InFlight> {
    Author(Pipeline::in_flight(client, pending))
}
