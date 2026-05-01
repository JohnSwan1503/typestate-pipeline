use super::{Hub, empty_order};

pub async fn main() {
    let hub = Hub;
    let pipeline = empty_order(&hub);

    let booked = pipeline
        .sku("widget".to_owned())
        .quantity(3)
        .book() // uses finalize() inside the transition body
        .await
        .expect("book ok");

    assert_eq!(booked.state().sku, "widget");
    assert_eq!(booked.state().quantity, 3);
    assert_eq!(booked.state().receipt_id, 1234);
}
