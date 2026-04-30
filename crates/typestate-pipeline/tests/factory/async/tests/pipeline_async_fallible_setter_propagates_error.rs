#[path = "shared.rs"]
mod shared;

use shared::{BadInput, Hub, empty_order};

pub async fn main() {
    let hub = Hub;
    let pipeline = empty_order(&hub);

    let result = pipeline
        .sku("X".to_owned())
        .quantity(0) // fails inside the async transformer
        .await;

    match result {
        Err(BadInput::Empty) => {}
        Ok(_) => panic!("expected BadInput::Empty"),
    }
}
