use super::{Server, empty_bag};

pub async fn main() {
    let server = Server::default();
    let pipeline = empty_bag(&server);

    // Set network, then drop it, then set it again — exercises the bag's
    // `drop_<field>` as a Pipeline-arm method.
    let pipeline = pipeline
        .name("ds-c".to_owned())
        .network("eth".to_owned())
        .drop_network()
        .network("op".to_owned())
        .label("primary".to_owned())
        .expect("label");

    let deployed = pipeline.deploy().await.expect("deploy");
    assert_eq!(deployed.state().dataset.network, "op");
}
