#[path = "shared.rs"]
mod shared;

use shared::UserProfileFactory;

pub async fn main() {
    let bag = UserProfileFactory::new()
        .name("  Alice  ".to_owned()) // async fn, returns Bag directly
        .await;

    // Now apply the fallible async setter.
    let bag = bag
        .email("Alice@Example.COM".to_owned()) // async fn returning Result<Bag, BadInput>
        .await
        .expect("non-empty");

    let user = bag.finalize();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
}
