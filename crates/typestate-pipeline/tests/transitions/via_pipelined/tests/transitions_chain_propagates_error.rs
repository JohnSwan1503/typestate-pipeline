#[path = "shared.rs"]
mod shared;

use shared::{AppError, Hub, drafted};

pub async fn main() {
    let hub = Hub;
    let initial = drafted(&hub, "alpha");

    // tag(0) fails — error must bubble through the chain.
    let result = initial.tag(0).publish().await;
    match result {
        Err(AppError::Bad(m)) => assert_eq!(m, "version must be > 0"),
        Ok(_) => panic!("expected error"),
    }
}
