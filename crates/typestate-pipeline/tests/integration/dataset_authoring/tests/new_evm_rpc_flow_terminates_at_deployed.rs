#[path = "shared.rs"]
mod shared;

use shared::{Client, NetworkId, Reference, Version, nm, ns};

pub async fn main() {
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
