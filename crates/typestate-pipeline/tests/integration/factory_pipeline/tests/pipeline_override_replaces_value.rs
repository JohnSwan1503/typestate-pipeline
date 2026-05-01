use super::{Server, empty_bag};

pub async fn main() {
    let server = Server::default();
    let pipeline = empty_bag(&server);

    let pipeline = pipeline
        .name("ds-d".to_owned())
        .network("eth".to_owned())
        .with_parallelism(2)
        .override_parallelism(8); // replaces 2 with 8, stays in `Yes`

    // Override flipped `parallelism`'s flag to `Yes`, but our `deploy`
    // transition is bounded on the flag = `No` (so finalize uses the
    // declared default). To exercise the override path, finalize the bag
    // directly. (The deploy transition's strict bound on the flag is
    // intentional — it shows you can declare exactly which bag shapes a
    // given transition operates on.)
    let pipeline = pipeline.label("primary".to_owned()).expect("label");

    let dataset = pipeline.into_state().finalize();
    assert_eq!(dataset.parallelism, 8);
}
