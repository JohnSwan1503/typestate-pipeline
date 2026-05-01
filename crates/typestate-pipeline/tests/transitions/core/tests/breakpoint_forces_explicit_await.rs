use typestate_pipeline::Resolved;

use super::{Author, Client, Versioned};

pub async fn main() {
    let client = Client::default();
    let pipeline = Author::from_registered(&client, "ds-b", 0xBEEF);

    // confirm_and_tag is `breakpoint` — its return type is
    // `Result<Author<Versioned, Resolved>, TestError>`, so the chain
    // breaks here and the next link is invoked on a Resolved pipeline.
    let versioned: Author<Versioned, Resolved> = pipeline
        .confirm_and_tag()
        .await
        .expect("breakpoint await failed");

    // After the breakpoint we're back in Resolved mode. Sync-fallible
    // `validate_and_finalize` correctly returns a `Result` and breaks
    // the chain — the user must handle it before chaining `.deploy()`.
    let configured = versioned
        .with_parallelism(4)
        .validate_and_finalize()
        .expect("validation failed");
    let deployed = configured.deploy().await.expect("deploy failed");

    assert_eq!(deployed.state().version, 1);
    assert_eq!(deployed.state().job_id, 1);
}
