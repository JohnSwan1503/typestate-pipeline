use typestate_pipeline::{BoxFuture, Resolved};

use super::{Author, Client, DummyError, Started, started_inflight};

pub async fn main() {
    let client = Client;
    let pending: BoxFuture<'_, Result<Started, DummyError>> = Box::pin(async { Ok(Started) });
    let in_flight = started_inflight(&client, pending);
    let resolved: Author<Started, Resolved> = in_flight.await.unwrap();
    let _ = resolved; // type-check only.
}
