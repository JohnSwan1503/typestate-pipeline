//! Minimal headline example for `typestate-pipeline`.
//!
//! Shows the three macros in their smallest useful configuration: one
//! factory bag, one carrier, one async transition. Run with
//! `cargo run --example minimal`.
//!
//! For the full feature surface — `pipeline(carrier = …)` composition,
//! the `<Bag>Ready` companion trait, async-fallible factory setters
//! that lift the chain to InFlight — see `examples/quickstart.rs`.

use typestate_pipeline::{Pipeline, TypestateFactory, pipelined, transitions};

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct Profile {
    #[field(required)]
    name: String,
    #[field(required)]
    email: String,
    #[field(default = 18)]
    age: u32,
}

#[derive(Debug)]
struct AuthError(&'static str);
impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
impl std::error::Error for AuthError {}

pipelined!(Author, ctx = (), error = AuthError);

struct Registered {
    profile: Profile,
}
struct Deployed {
    profile: Profile,
    account_id: u64,
}

#[transitions]
impl<'a> Author<'a, Registered> {
    #[transition(into = Deployed)]
    pub async fn deploy(state: Registered) -> Result<Deployed, AuthError> {
        Ok(Deployed {
            profile: state.profile,
            account_id: 42,
        })
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), AuthError> {
    let profile = ProfileFactory::new()
        .name("Alice".into())
        .email("alice@example.com".into())
        .with_age(30)
        .finalize();

    let deployed = Author(Pipeline::resolved(&(), Registered { profile }))
        .deploy()
        .await?;

    let state = deployed.0.into_state();
    println!("{} got account #{}", state.profile.name, state.account_id);
    Ok(())
}
