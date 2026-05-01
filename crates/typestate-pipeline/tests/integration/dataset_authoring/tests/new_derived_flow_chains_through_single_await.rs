use super::{Client, TableName, Version, nm, ns};

pub async fn main() {
    let client = Client::default();

    let deployed = client
        .author()
        .new_derived(ns("eth"), nm("transactions_opt"))
        .into_builder()
        .add_dependency("alias".into(), "pkg:amp/eth/blocks@0.1.0".into())
        .add_table(
            TableName("opt".into()),
            "SELECT * FROM raw.transactions".into(),
        )
        .register()
        .tag_version(Version::new(0, 1, 0))
        .with_parallelism(4)
        .deploy()
        .await
        .expect("deploy");

    assert_eq!(deployed.job_id().0, 1);
}
