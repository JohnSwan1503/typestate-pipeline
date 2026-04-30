use core::fmt;

use typestate_pipeline::{Pipeline, Resolved, TypestateFactory, pipelined};

#[derive(Debug, Default)]
struct Hub;

#[derive(Debug)]
struct AppError;
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("app error")
    }
}
impl std::error::Error for AppError {}

pipelined!(Author, ctx = Hub, error = AppError);

#[derive(TypestateFactory)]
#[factory(error = AppError, pipeline(carrier = Author))]
#[allow(dead_code)]
struct Order {
    #[field(required)]
    sku: String,
    #[field(default = 1, overridable)]
    quantity: u32,
}

fn main() {
    let hub = Hub;
    let carrier: Author<OrderFactory, Resolved> =
        Author(Pipeline::resolved(&hub, OrderFactory::new()));

    // Drive the bag through the carrier — no `Pipeline::resolved` /
    // `into_state()` plumbing in user code.
    let carrier = carrier
        .sku("widget".to_owned())
        .with_quantity(2)
        .override_quantity(5); // overridable arm on the carrier

    // The carrier-side getter is callable once the field's flag is Yes.
    assert_eq!(*carrier.0.state().sku(), "widget");

    let order = carrier.0.into_state().finalize();
    assert_eq!(order.sku, "widget");
    assert_eq!(order.quantity, 5);
}
