use typestate_pipeline::Resolved;

use super::{Author, Client, JobConfigured};

pub async fn main() {
    let client = Client::default();
    let pipeline = Author::from_registered(&client, "ds-c", 1)
        .confirm_and_tag()
        .await
        .unwrap();

    // Resolved → Resolved: sync fallible returns `Result<…, TestError>`
    // on the Resolved arm, no future indirection.
    let configured: Author<JobConfigured, Resolved> = pipeline
        .with_parallelism(2)
        .validate_and_finalize()
        .unwrap();

    assert!(configured.state().verified);
    assert_eq!(configured.state().parallelism, 2);
}
