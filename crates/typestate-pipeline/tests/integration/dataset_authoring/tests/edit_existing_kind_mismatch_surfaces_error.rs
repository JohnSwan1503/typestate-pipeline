#[path = "shared.rs"]
mod shared;

use shared::{Client, Reference, Version, nm, ns};

pub async fn main() {
    let client = Client::default();

    // Register as evm_rpc, then try to edit as derived — should fail.
    let body = serde_json::value::RawValue::from_string("{}".into()).unwrap();
    client
        .register(ns("eth"), nm("blocks"), body, "evm_rpc")
        .await;

    let reference = Reference {
        namespace: ns("eth"),
        name: nm("blocks"),
        version: Version::new(0, 1, 0),
    };

    let result = client
        .author()
        .edit_existing_derived(&reference)
        .into_builder()
        .register()
        .await;

    assert!(matches!(
        result,
        Err(
            typestate_pipeline::dataset_authoring::error::AuthoringError::KindMismatch {
                expected: "derived",
                ..
            }
        )
    ));
}
