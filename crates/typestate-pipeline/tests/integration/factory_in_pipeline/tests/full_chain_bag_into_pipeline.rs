#[path = "shared.rs"]
mod shared;

use typestate_pipeline::Resolved;

use shared::{Author, Confirmed, Server, User, drafting};

pub async fn main() {
    let server = Server::default();
    let pipeline = drafting(
        &server,
        User {
            name: "Alice".to_owned(),
            email: "alice@example.com".to_owned(),
            age: 30,
        },
    );

    // Sync fallible (submit) folds its `Result` into the InFlight chain that
    // confirm() opens — single terminal `.await?`.
    let confirmed: Author<Confirmed, Resolved> = pipeline
        .submit() // Result<Author<Submitted, Resolved>, SubmitError>
        .expect("submit should succeed")
        .confirm() // Author<Confirmed, InFlight>
        .await
        .expect("confirm should succeed");

    assert_eq!(confirmed.state().user.name, "Alice");
    assert_eq!(confirmed.state().confirmation_id, 1);
}
