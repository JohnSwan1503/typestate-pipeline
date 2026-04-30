#[path = "shared.rs"]
mod shared;

use shared::{Hub, carrier};

pub fn main() {
    // After setting `parallelism` on the carrier, the carrier-arm
    // getter for `parallelism` is callable. Verify reads match what
    // was set.
    let hub = Hub;
    let pipeline = carrier(&hub, "eth");

    let configured = pipeline.parallelism(16);
    assert_eq!(*configured.parallelism(), 16);
    // Internal getter still works on the configured carrier — the
    // impl block for the internal getter is parameterized over every
    // flag, so it doesn't restrict which "shape" of the bag the
    // carrier is in.
    assert_eq!(configured.namespace(), "eth");
}
