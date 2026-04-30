use core::fmt;

use typestate_pipeline::{Pipeline, Resolved, pipelined, transitions};

#[derive(Debug, Default)]
struct Hub;

#[derive(Debug)]
struct AppError;
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("err")
    }
}
impl std::error::Error for AppError {}

pipelined!(Author, ctx = Hub, error = AppError);

#[derive(Debug)]
struct Registered {
    name: String,
}

#[derive(Debug)]
struct Versioned {
    name: String,
    version: u32,
}

#[derive(Debug)]
struct Deployed {
    name: String,
    version: u32,
}

#[transitions(error = AppError)]
impl<'a> Author<'a, Registered> {
    #[transition(into = Versioned)]
    pub async fn tag_version(
        state: Registered,
        ctx: &Hub,
        version: u32,
    ) -> Result<Versioned, AppError> {
        let _ = ctx;
        tokio::task::yield_now().await;
        Ok(Versioned {
            name: state.name,
            version,
        })
    }
}

#[transitions(error = AppError)]
impl<'a> Author<'a, Versioned> {
    #[transition(into = Deployed)]
    pub async fn deploy(state: Versioned, ctx: &Hub) -> Result<Deployed, AppError> {
        let _ = ctx;
        tokio::task::yield_now().await;
        Ok(Deployed {
            name: state.name,
            version: state.version,
        })
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let hub = Hub;
    let carrier: Author<Registered, Resolved> = Author(Pipeline::resolved(
        &hub,
        Registered {
            name: "ds".to_owned(),
        },
    ));

    // Two async transitions; one terminal `.await?` drives both.
    let deployed: Author<Deployed, Resolved> = carrier
        .tag_version(7)  // Resolved -> InFlight
        .deploy()        // InFlight  -> InFlight (folds onto the chain)
        .await           // InFlight  -> Resolved (drives the chain)
        .expect("chain ok");

    let state = deployed.0.into_state();
    assert_eq!(state.name, "ds");
    assert_eq!(state.version, 7);
}
