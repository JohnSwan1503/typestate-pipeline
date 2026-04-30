#[path = "shared.rs"]
mod shared;

use shared::{Hub, carrier};

pub fn main() {
    // The carrier-arm getter for an internal field is callable on
    // every Resolved-mode bag carrier — same shape as the standalone,
    // just delegating through `self.0.state()`.
    let hub = Hub;
    let c = carrier(&hub, "op");
    assert_eq!(c.namespace(), "op");
}
