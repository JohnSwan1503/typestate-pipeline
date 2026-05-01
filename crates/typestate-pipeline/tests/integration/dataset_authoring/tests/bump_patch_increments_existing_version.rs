use super::{Client, Reference, TableName, Version, nm, ns};

pub async fn main() {
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
