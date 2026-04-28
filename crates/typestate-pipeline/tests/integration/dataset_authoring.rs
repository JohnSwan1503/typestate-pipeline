//! End-to-end integration tests for the rebuilt dataset-authoring pipeline.
//!
//! Mirrors the four upstream `amp-client-admin` example flows, but as
//! tests so the chain shapes can be regression-locked. Each test asserts a
//! concrete observable property (job id allocation, version bump
//! arithmetic, deployed parallelism) on top of the in-memory mock client.
//!
//! Gated behind the same `dataset-authoring-example` feature as the demo
//! example so the optional `serde`/`serde_json`/`thiserror` deps don't
//! pollute normal `cargo test` runs.

#![cfg(feature = "dataset-authoring-example")]

use typestate_pipeline::dataset_authoring::{
    client::Client,
    primitives::{Name, Namespace, NetworkId, Reference, TableName, Version},
};

fn ns(s: &str) -> Namespace {
    Namespace(s.to_owned())
}
fn nm(s: &str) -> Name {
    Name(s.to_owned())
}

#[tokio::test]
async fn new_evm_rpc_flow_terminates_at_deployed() {
    let client = Client::default();

    let deployed = client
        .author()
        .new_evm_rpc(ns("eth"), nm("blocks"))
        .into_builder()
        .with_finalized_blocks_only(true)
        .with_start_block(42)
        .with_network(NetworkId("mainnet".into()))
        .with_default_tables()
        .register()
        .tag_version(Version::new(0, 1, 0))
        .with_verify(true)
        .with_parallelism(2)
        .deploy()
        .await
        .expect("deploy");

    assert_eq!(deployed.job_id().0, 1);
    assert_eq!(
        deployed.reference(),
        Reference {
            namespace: ns("eth"),
            name: nm("blocks"),
            version: Version::new(0, 1, 0),
        }
    );
}

#[tokio::test]
async fn new_derived_flow_chains_through_single_await() {
    let client = Client::default();

    let deployed = client
        .author()
        .new_derived(ns("eth"), nm("transactions_opt"))
        .into_builder()
        .add_dependency("alias".into(), "pkg:amp/eth/blocks@0.1.0".into())
        .add_table(TableName("opt".into()), "SELECT * FROM raw.transactions".into())
        .register()
        .tag_version(Version::new(0, 1, 0))
        .with_parallelism(4)
        .deploy()
        .await
        .expect("deploy");

    assert_eq!(deployed.job_id().0, 1);
}

#[tokio::test]
async fn bump_patch_increments_existing_version() {
    let client = Client::default();

    // Seed: a derived dataset already exists at 0.1.0.
    let body = serde_json::value::RawValue::from_string("{}".into()).unwrap();
    client
        .register(ns("eth"), nm("transactions_opt"), body, "derived")
        .await;
    client
        .tag(ns("eth"), nm("transactions_opt"), Version::new(0, 1, 0))
        .await;

    let reference = Reference {
        namespace: ns("eth"),
        name: nm("transactions_opt"),
        version: Version::new(0, 1, 0),
    };

    let deployed = client
        .author()
        .edit_existing_derived(&reference)
        .into_builder()
        .add_table(TableName("extra".into()), "SELECT 1".into())
        .register()
        .bump_patch()
        .with_parallelism(10)
        .deploy()
        .await
        .expect("deploy");

    assert_eq!(deployed.reference().version, Version::new(0, 1, 1));
}

#[tokio::test]
async fn bump_patch_errors_when_no_prior_version() {
    let client = Client::default();

    // Register without tagging any version — bump_patch should fail.
    let body = serde_json::value::RawValue::from_string("{}".into()).unwrap();
    client.register(ns("eth"), nm("untagged"), body, "derived").await;

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

#[tokio::test]
async fn edit_existing_kind_mismatch_surfaces_error() {
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
        Err(typestate_pipeline::dataset_authoring::error::AuthoringError::KindMismatch {
            expected: "derived",
            ..
        })
    ));
}
