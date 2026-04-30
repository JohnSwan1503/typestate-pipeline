#[path = "shared.rs"]
mod shared;

use typestate_pipeline::Resolved;

use shared::{Author, Hub, OrderFactory, empty_order};

pub async fn main() {
    let hub = Hub;
    let pipeline = empty_order(&hub);

    // The async setter `sku` opens an InFlight chain (Resolved →
    // InFlight). Subsequent setters chain through the pending future.
    // Only one terminal `.await?` is required to drive the entire
    // chain.
    let bag: Author<OrderFactory<_, _>, Resolved> = pipeline
        .sku("  SKU-42  ".to_owned()) // Resolved → InFlight (async deferred)
        .quantity(5) // chains through InFlight
        .await
        .expect("chain ok");

    // Confirm the values were trimmed + validated.
    let raw = bag.into_state().finalize();
    assert_eq!(raw.sku, "SKU-42");
    assert_eq!(raw.quantity, 5);
}
