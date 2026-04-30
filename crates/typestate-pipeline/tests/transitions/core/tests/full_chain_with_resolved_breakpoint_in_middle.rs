#[path = "shared.rs"]
mod shared;

use shared::{Author, Client, Deployed};

pub async fn main() {
    let client = Client::default();
    let pipeline = Author::from_registered(&client, "ds-a", 0xCAFE);

    // Resolved-direct: tag_version returns InFlight (lifting is the default for `async fn`)
    // folds into the rest of the chain — single terminal `.await?`.
    let deployed: Author<Deployed> = pipeline
        .tag_version(7)
        .with_parallelism(8)
        .validate_and_finalize() // sync fallible — folds Result into pending
        .deploy()
        .await
        .expect("chain failed");

    let state = deployed.state().clone();
    assert_eq!(state.name, "ds-a");
    assert_eq!(state.version, 7);
    assert_eq!(state.job_id, 1);
}
