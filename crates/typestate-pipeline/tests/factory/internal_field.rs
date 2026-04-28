//! Verify `#[field(internal)]` — fields supplied positionally at the
//! constructor and locked in from then on.
//!
//! Positive checks:
//! 1. `JobFactory::new(namespace)` accepts the internal field as a
//!    constructor argument; the resulting bag is in the all-deploy-flag-No
//!    state and ready to drive through user-facing setters.
//! 2. The internal field has a getter that's callable on every bag
//!    instantiation (no flag bound).
//! 3. The bag finalizes correctly with the internal field always present.
//! 4. Internal fields don't appear in the bag's flag generic list (witnessed
//!    by `JobFactory<No, No>` accepting only the *non-internal* flags).
//!
//! Negative checks (no setter, no overrider, no remover, no pipeline arm
//! for the internal field) live in the UI test suite.

use core::fmt;

use typestate_pipeline::{No, Pipeline, Resolved, TypestateFactory, pipelined};

#[derive(Debug, Default)]
struct Hub;

#[derive(Debug)]
enum AppError {}
impl fmt::Display for AppError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
impl std::error::Error for AppError {}

pipelined!(Author, ctx = Hub, error = AppError);

#[derive(Debug, TypestateFactory)]
#[factory(pipeline(carrier = Author))]
struct Job {
    /// Phase-boundary plumbing — set positionally on `new(…)`.
    #[field(required, internal)]
    namespace: String,

    /// User-facing.
    #[field(required)]
    parallelism: u16,

    /// User-facing, optional with default.
    #[field(default = false)]
    verify: bool,
}

#[test]
fn constructor_takes_internal_field_as_argument() {
    // No `.namespace(…)` chain — the field is supplied to `new(…)`.
    let bag = JobFactory::new("eth".to_owned()).parallelism(4).with_verify(true);
    let job = bag.finalize();
    assert_eq!(job.namespace, "eth");
    assert_eq!(job.parallelism, 4);
    assert!(job.verify);
}

#[test]
fn internal_getter_is_unconditional() {
    // Bag in the all-No-flags state still has `namespace()` callable —
    // the internal field has no `Yes`-required bound.
    let bag = JobFactory::new("op".to_owned());
    assert_eq!(bag.namespace(), "op");
}

#[test]
fn internal_field_dropped_from_flag_generic_list() {
    // The bag's type signature is `JobFactory<ParallelismFlag, VerifyFlag>`
    // — only two generics, NOT three. If the internal field had a flag,
    // the type below would need `JobFactory<Yes, No, No>` instead. The
    // fact that this annotation typechecks is the witness.
    let bag: JobFactory<No, No> = JobFactory::new("eth".to_owned());
    let _ = bag;
}

#[test]
fn pipeline_arm_works_for_non_internal_fields() {
    // Open the pipeline carrier with the internal field already populated
    // (mimicking what a transition body would do), then drive the
    // user-facing fields via the auto-generated pipeline arms.
    let hub = Hub;
    let pipeline =
        Author(Pipeline::resolved(&hub, JobFactory::new("eth".to_owned())));

    let chain: Author<_, Resolved> = pipeline.parallelism(8).with_verify(true);
    let job = chain.0.into_state().finalize();

    assert_eq!(job.namespace, "eth");
    assert_eq!(job.parallelism, 8);
    assert!(job.verify);
}

#[test]
fn finalize_passes_internal_field_through() {
    // Internal fields are read directly out of plain-`T` storage by
    // finalize — no MaybeUninit unwrap, no flag check.
    let job = JobFactory::new("solana".to_owned()).parallelism(1).finalize();
    assert_eq!(job.namespace, "solana");
}

#[test]
fn carrier_internal_getter_is_unconditional() {
    // The carrier-arm getter for an internal field is callable on every
    // Resolved-mode bag carrier — same shape as the standalone, just
    // delegating through `self.0.state()`.
    let hub = Hub;
    let carrier =
        Author(Pipeline::resolved(&hub, JobFactory::new("op".to_owned())));
    assert_eq!(carrier.namespace(), "op");
}

#[test]
fn carrier_non_internal_getter_gates_on_yes_flag() {
    // After setting `parallelism` on the carrier, the carrier-arm getter
    // for `parallelism` is callable. Verify reads match what was set.
    let hub = Hub;
    let pipeline =
        Author(Pipeline::resolved(&hub, JobFactory::new("eth".to_owned())));

    let configured = pipeline.parallelism(16);
    assert_eq!(*configured.parallelism(), 16);
    // Internal getter still works on the configured carrier — the impl
    // block for the internal getter is parameterized over every flag,
    // so it doesn't restrict which "shape" of the bag the carrier is in.
    assert_eq!(configured.namespace(), "eth");
}
