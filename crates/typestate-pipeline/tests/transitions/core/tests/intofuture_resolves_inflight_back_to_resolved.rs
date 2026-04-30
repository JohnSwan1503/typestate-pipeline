#[path = "shared.rs"]
mod shared;

use typestate_pipeline::Resolved;

use shared::{Author, Client, Versioned};

pub async fn main() {
    let client = Client::default();
    let pipeline = Author::from_registered(&client, "ds-e", 9).tag_version(3); // InFlight

    // Awaiting an InFlight pipeline directly hands back a Resolved one.
    let versioned: Author<Versioned, Resolved> = pipeline.await.unwrap();
    assert_eq!(versioned.state().version, 3);
    assert_eq!(versioned.state().manifest_hash, 9);
}
