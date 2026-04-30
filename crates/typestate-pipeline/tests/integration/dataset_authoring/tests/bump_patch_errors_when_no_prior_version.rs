#[path = "shared.rs"]
mod shared;

use shared::{Client, Reference, Version, nm, ns};

pub async fn main() {
    let client = Client::default();

    // Register without tagging any version — `bump_patch` should fail.
    let body = serde_json::value::RawValue::from_string("{}".into()).unwrap();
    client
        .register(ns("eth"), nm("untagged"), body, "derived")
        .await;

    let reference = Reference {
        namespace: ns("eth"),
        name: nm("untagged"),
        version: Version::new(0, 0, 0),
    };

    let result = client
        .author()
        .edit_existing_derived(&reference)
        .into_builder()
        .register()
        .bump_patch()
        .with_parallelism(1)
        .deploy()
        .await;

    assert!(matches!(
        result,
        Err(typestate_pipeline::dataset_authoring::error::AuthoringError::NoPriorVersion)
    ));
}
