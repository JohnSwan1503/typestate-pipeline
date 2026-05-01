use std::future::IntoFuture;

use super::{AppError, Author, Drafted, Hub, drafted_inflight};
use typestate_pipeline::Resolved;

pub async fn main() {
    let hub = Hub;
    let pending: typestate_pipeline::BoxFuture<'_, Result<Drafted, AppError>> = Box::pin(async {
        Ok(Drafted {
            name: "x".to_owned(),
        })
    });
    let in_flight = drafted_inflight(&hub, pending);
    let resolved: Author<Drafted, Resolved> = in_flight.into_future().await.unwrap();
    assert_eq!(resolved.into_state().name, "x");
}
