#[path = "shared.rs"]
mod shared;

use shared::{Hub, empty_profile};

pub async fn main() {
    // Round-trip through the carrier: open a Resolved Author with an
    // empty bag, drive setters via the generated pipeline arms,
    // finalize.
    let hub = Hub;
    let initial = empty_profile(&hub);

    let bag_carrier = initial
        .handle("alice".to_owned()) // pipeline-integrated setter
        .age_default(); // pipeline-integrated default helper

    let profile = bag_carrier.into_state().finalize();
    assert_eq!(profile.handle, "alice");
    assert_eq!(profile.age, 0);
}
