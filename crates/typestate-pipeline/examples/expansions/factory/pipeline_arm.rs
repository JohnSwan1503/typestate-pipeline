//! `#[factory(pipeline(carrier = MyCarrier))]` instructs the derive to
//! *also* emit setters / removers / overriders / default helpers /
//! getters on the user's `Pipeline`-newtype carrier — so callers can
//! build a bag *through* the pipeline carrier in either Resolved or
//! InFlight mode without `into_state()` / `Pipeline::resolved` plumbing.
//!
//! Each method is emitted twice:
//!
//! - on `Carrier<'a, Bag<…>, Resolved>`: takes the carrier, returns
//!   the carrier with the new bag state.
//! - on `Carrier<'a, Bag<…>, InFlight>`: takes the in-flight carrier,
//!   returns the in-flight carrier with the new bag state — fallible
//!   methods fold the `Result` into the pending future.
//!
//! Getters on the carrier are Resolved-only (no synchronous access to
//! an in-flight state).
//!
//! =============================================================================
//! Generated (sketch) — addition to standalone setters
//! =============================================================================
//!
//!     // Resolved-mode setter on the carrier.
//!     impl<'a, F2> Author<'a, OrderFactory<No, F2>, Resolved> {
//!         pub fn sku(self, val: String) -> Author<'a, OrderFactory<Yes, F2>, Resolved>;
//!     }
//!
//!     // InFlight-mode setter on the carrier (Send/Sync-bounded).
//!     impl<'a, F2> Author<'a, OrderFactory<No, F2>, InFlight>
//!     where
//!         OrderFactory<No, F2>: Send + 'a,
//!         OrderFactory<Yes, F2>: Send + 'a,
//!     {
//!         pub fn sku(self, val: String) -> Author<'a, OrderFactory<Yes, F2>, InFlight>;
//!     }
//!
//!     // … same shape for `with_<f>`, `<f>_default`, `drop_<f>`, `override_<f>`.
//!     // Resolved-only getter:
//!     impl<'a, F2> Author<'a, OrderFactory<Yes, F2>, Resolved> {
//!         pub fn sku(&self) -> &String;
//!     }

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
