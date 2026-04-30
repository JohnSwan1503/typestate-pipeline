#[path = "shared.rs"]
mod shared;

use shared::{Hub, drafted};

pub async fn main() {
    let hub = Hub;
    let initial = drafted(&hub, "alpha");

    // Resolved → InFlight (deferred async) → InFlight (sync fallible
    // folds in) → terminal `.await?`.
    let published = initial
        .tag(7)
        .publish()
        .await
        .expect("chain should succeed");

    let state = published.into_state();
    assert_eq!(state.name, "alpha");
    assert_eq!(state.version, 7);
}
