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
#[allow(dead_code)]
struct Versioned {
    name: String,
    version: u32,
}

#[transitions(error = AppError)]
impl<'a> Author<'a, Registered> {
    /// Breakpoint — caller must await this before the next link.
    #[transition(into = Versioned, breakpoint)]
    pub async fn confirm_and_tag(
        state: Registered,
        ctx: &Hub,
    ) -> Result<Versioned, AppError> {
        let _ = ctx;
        tokio::task::yield_now().await;
        Ok(Versioned {
            name: state.name,
            version: 1,
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

    // Breakpoint: the chain pauses at the .await — the next link sees a
    // Resolved carrier whose state is fully observable.
    let versioned: Author<Versioned, Resolved> =
        carrier.confirm_and_tag().await.expect("confirm");

    assert_eq!(versioned.0.into_state().version, 1);
}
