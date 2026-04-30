#[path = "shared.rs"]
mod shared;

use shared::{ConfirmedUser, UserFactory};

pub async fn main() {
    let bag = UserFactory::new().name("Carol".to_owned());

    // Sync finalize is still available.
    let raw = bag.finalize();
    assert_eq!(raw.name, "Carol");

    // Async finalize calls the hook.
    let confirmed = UserFactory::new()
        .name("Carol".to_owned())
        .finalize_async()
        .await
        .expect("hook ok");
    assert_eq!(
        confirmed,
        ConfirmedUser {
            name: "Carol".to_owned(),
            confirmation_token: "token-for-Carol".to_owned()
        }
    );
}
