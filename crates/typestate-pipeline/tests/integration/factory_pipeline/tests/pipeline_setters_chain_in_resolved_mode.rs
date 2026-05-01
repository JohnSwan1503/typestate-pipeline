use super::{Server, empty_bag};

pub async fn main() {
    let server = Server::default();
    let pipeline = empty_bag(&server);

    // All setters are direct methods on the Author carrier — no
    // .into_future() in the middle, all sync.
    let bag_full = pipeline
        .name("ds-a".to_owned())
        .network("eth".to_owned())
        // skip parallelism — let it default at finalize
        .label("  primary  ".to_owned()) // fallible — Result<Author, AppError>
        .expect("label trim should succeed");

    let deployed = bag_full.deploy().await.expect("deploy");
    assert_eq!(deployed.state().dataset.name, "ds-a");
    assert_eq!(deployed.state().dataset.parallelism, 4); // default
    assert_eq!(deployed.state().dataset.label, "primary"); // trimmed
    assert_eq!(deployed.state().job_id, 1);
}
