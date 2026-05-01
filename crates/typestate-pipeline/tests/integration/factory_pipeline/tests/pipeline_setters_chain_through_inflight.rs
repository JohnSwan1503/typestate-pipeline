use super::{Author, Server, empty_inflight_bag};
use typestate_pipeline::Resolved;

pub async fn main() {
    let server = Server::default();

    // No transition in this carrier opens an InFlight chain on its own
    // (the only async transition is `deploy`, which is terminal).
    // `empty_inflight_bag` constructs an InFlight carrier by hand so we can
    // exercise the InFlight arms of the bag's setters directly.
    let initial = empty_inflight_bag(&server);

    // On InFlight, the fallible setter folds its `Result` into the chained
    // pending future — no `Result` at the call site, just a chained InFlight.
    let chain = initial
        .name("ds-b".to_owned())
        .network("eth".to_owned())
        .label("  alpha  ".to_owned()); // InFlight — Result is folded

    // Awaiting the InFlight chain hands back a Resolved bag.
    let resolved: Author<_, Resolved> = chain.await.expect("chain");

    let deployed = resolved.deploy().await.expect("deploy");
    assert_eq!(deployed.state().dataset.label, "alpha");
}
