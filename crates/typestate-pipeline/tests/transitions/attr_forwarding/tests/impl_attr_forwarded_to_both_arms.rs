use typestate_pipeline::{Pipeline, Resolved, pipelined, transitions};

#[derive(Debug, Default)]
struct Hub;

#[derive(Debug)]
enum AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl std::error::Error for AppError {}

pipelined!(Author, ctx = Hub, error = AppError);

#[derive(Debug)]
struct Started;

#[derive(Debug)]
struct Finished;

// `#[cfg(all())]` is the always-true cfg. Used as a stand-in for any
// user-level cfg gate. The point is that the macro preserves the attr
// and applies it to BOTH the Resolved and InFlight arms, not just one
// of them. Clippy's `non_minimal_cfg` flags `cfg(all())` as redundant —
// which it is in normal code, but here we deliberately want to verify
// cfg propagation, so suppress the lint.
#[allow(clippy::non_minimal_cfg)]
#[cfg(all())]
#[transitions]
impl<'a> Author<'a, Started> {
    #[transition(into = Finished)]
    pub fn finish(state: Started) -> Finished {
        let _ = state;
        Finished
    }
}

pub async fn main() {
    let hub = Hub;

    // Resolved arm — calling on the Resolved carrier compiles, proving
    // the Resolved arm survived the `#[cfg(all())]` gate.
    let resolved = Author(Pipeline::resolved(&hub, Started)).finish();
    let _: Author<Finished, Resolved> = resolved;

    // InFlight arm — calling on the InFlight carrier compiles, proving
    // the InFlight arm also survived. If only one arm received the cfg
    // attr we'd get either dead code or a compile error here.
    let inflight = Author(Pipeline::in_flight(&hub, Box::pin(async { Ok(Started) }))).finish();
    let resolved = inflight.await.unwrap();
    let _: Author<Finished, Resolved> = resolved;
}
