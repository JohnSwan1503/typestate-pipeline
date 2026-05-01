use typestate_pipeline::Resolved;

use super::{Author, Hub, carrier};

pub fn main() {
    // Open the pipeline carrier with the internal field already
    // populated (mimicking what a transition body would do), then
    // drive the user-facing fields via the auto-generated pipeline
    // arms.
    let hub = Hub;
    let pipeline = carrier(&hub, "eth");

    let chain: Author<_, Resolved> = pipeline.parallelism(8).with_verify(true);
    let job = chain.into_state().finalize();

    assert_eq!(job.namespace, "eth");
    assert_eq!(job.parallelism, 8);
    assert!(job.verify);
}
