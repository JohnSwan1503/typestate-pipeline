use core::fmt;

use typestate_pipeline::TypestateFactory;

#[derive(Debug)]
struct BadInput;

impl fmt::Display for BadInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("bad input")
    }
}
impl std::error::Error for BadInput {}

#[derive(TypestateFactory)]
#[factory(error = BadInput)]
#[allow(dead_code)]
struct User {
    #[field(required, setter = normalize_async, async_fn)]
    name: String,
    #[field(required, setter = validate_email_async, async_fn, fallible)]
    email: String,
}

async fn normalize_async(value: String) -> String {
    tokio::task::yield_now().await;
    value.trim().to_owned()
}

async fn validate_email_async(value: String) -> Result<String, BadInput> {
    tokio::task::yield_now().await;
    if value.is_empty() {
        Err(BadInput)
    } else {
        Ok(value.to_lowercase())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let user = UserFactory::new()
        .name("  Alice  ".to_owned()) // async — flips name flag after .await
        .await
        .email("Alice@Example.COM".to_owned()) // async fallible — Result after .await
        .await
        .expect("non-empty")
        .finalize();

    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
}
