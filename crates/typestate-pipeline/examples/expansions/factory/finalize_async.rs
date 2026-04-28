//! `#[factory(finalize_async(via = my_fn, into = Target, error = E?))]`
//! emits an `async fn finalize_async()` on the bag, callable on the same
//! shapes as `finalize()`. The body is `my_fn(self.finalize()).await`,
//! so `my_fn` receives the assembled raw struct and produces the final
//! value (with or without an error).
//!
//! `finalize_async` does **not** replace the inherent `finalize()` —
//! both coexist. Use whichever fits the call site.
//!
//! =============================================================================
//! Generated (sketch) — diff from baseline (see `./minimal.rs`)
//! =============================================================================
//!
//!     impl UserFactory<Yes> {
//!         pub async fn finalize_async(self) -> Result<ConfirmedUser, BadInput>;
//!         //                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!         //   omit `error = …` to drop the `Result` wrapper:
//!         //   pub async fn finalize_async(self) -> ConfirmedUser;
//!     }

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

#[derive(Debug, PartialEq, Eq)]
struct ConfirmedUser {
    name: String,
    confirmation_token: String,
}

async fn confirm_user(raw: User) -> Result<ConfirmedUser, BadInput> {
    tokio::task::yield_now().await;
    Ok(ConfirmedUser {
        confirmation_token: format!("token-for-{}", raw.name),
        name: raw.name,
    })
}

#[derive(TypestateFactory)]
#[factory(
    error = BadInput,
    finalize_async(via = confirm_user, into = ConfirmedUser, error = BadInput),
)]
#[allow(dead_code)]
struct User {
    #[field(required)]
    name: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Sync finalize is still available.
    let raw = UserFactory::new().name("Carol".to_owned()).finalize();
    assert_eq!(raw.name, "Carol");

    // Async finalize routes through `confirm_user`.
    let confirmed = UserFactory::new()
        .name("Carol".to_owned())
        .finalize_async()
        .await
        .expect("hook ok");

    assert_eq!(
        confirmed,
        ConfirmedUser {
            name: "Carol".to_owned(),
            confirmation_token: "token-for-Carol".to_owned(),
        }
    );
}
