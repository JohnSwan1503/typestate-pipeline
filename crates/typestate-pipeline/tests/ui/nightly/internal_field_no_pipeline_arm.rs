//! Confirms `#[field(internal)]` suppresses the *setter* on the carrier
//! while still exposing the *getter*. Calling the field name with a value
//! argument must fail — the getter takes zero args, and no setter exists.
//!
//! Reading `bag.namespace()` (no args) would compile and return `&String`;
//! that path is exercised positively in `tests/factory/internal_field.rs`.

use core::fmt;

use typestate_pipeline::{Pipeline, Resolved, TypestateFactory, pipelined};

#[derive(Debug)]
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

#[derive(TypestateFactory)]
#[factory(pipeline(carrier = Author))]
struct Job {
    #[field(required, internal)]
    namespace: String,
    #[field(required)]
    parallelism: u16,
}

fn main() {
    let hub = Hub;
    // `JobFactory::new` takes the internal `namespace` positionally.
    let bag = Author(Pipeline::resolved(
        &hub,
        JobFactory::new("eth".to_owned()),
    ));
    let bag: Author<_, Resolved> = bag;
    // ERROR: `namespace` is `internal`, so no setter was emitted on
    // either the bag or the carrier.
    let _ = bag.namespace("override".to_owned());
}
