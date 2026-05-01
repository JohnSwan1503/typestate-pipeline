//! Headline quickstart for `typestate-pipeline`.
//!
//! Mirrors the README's first example. Shows every macro and the most
//! common field attributes in one ~75-line program. Run with
//! `cargo run --example quickstart`. For a multi-phase pipeline that
//! exercises chain folding across many transitions, see
//! `examples/dataset_authoring.rs`.

use typestate_pipeline::{Pipeline, TypestateFactory, pipelined, transitions};

#[derive(Debug)]
struct Error(&'static str);
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
impl std::error::Error for Error {}

// One derive emits a sister bag type — here renamed `Settings` while
// the underlying data type stays `SettingsData`. Setters live on the
// bag; `finalize()` on the bag returns a `SettingsData`. The
// `pipeline(carrier = Worker)` arm also emits the bag's setters
// directly on `Worker` in both Resolved and InFlight modes.
#[derive(TypestateFactory)]
#[factory(name = Settings, error = Error, pipeline(carrier = Worker))]
#[allow(dead_code)]
struct SettingsData {
    #[field(internal)]
    workspace_id: u64, // positional on `Settings::new(...)`, no setter
    #[field(required, setter = validate_endpoint, async_fn, fallible)]
    endpoint: String, // `.endpoint("...")` — async-fallible; lifts the chain to InFlight
    #[field(default = 30)]
    timeout_secs: u32, // `.with_timeout_secs(60)` — defaults to 30
    #[field(default, removable, overridable)]
    label: String, // also `.drop_label()` / `.override_label(...)`
}

// Async-fallible transformer for the `endpoint` setter. The async setter
// on a `pipeline(carrier = ...)` arm lifts the chain to InFlight, so
// every following step — including other factory setters — folds into
// the same pending future and resolves at the chain's terminal `.await?`.
async fn validate_endpoint(value: String) -> Result<String, Error> {
    tokio::task::yield_now().await;
    if value.starts_with("https://") {
        Ok(value)
    } else {
        Err(Error("endpoint must use https"))
    }
}

// One line declares the `Worker` newtype around `Pipeline`, the
// `Pipelined<'a>` impl, and `IntoFuture` forwarding for InFlight mode.
pipelined!(Worker, ctx = (), error = Error);

#[derive(Default)]
struct Idle;
struct Running {
    id: u64,
}

// Each `#[transition(into = Next)]` body becomes a Resolved + InFlight
// method pair, with the arrow chosen from the body shape.
#[transitions]
impl<'a> Worker<'a, Idle> {
    /// Sync infallible — Resolved → Resolved. Enters the configuration
    /// phase with a fresh all-`No` bag. `SettingsEmpty` is the
    /// auto-generated alias for `Settings<No, No, No>` — the entry-side
    /// counterpart to `SettingsReady`.
    #[transition(into = SettingsEmpty)]
    fn configure(state: Idle, workspace_id: u64) -> SettingsEmpty {
        Settings::new(workspace_id)
    }
}

// `B: SettingsReady` is the auto-generated companion trait that
// abstracts over every flag-tuple where `finalize()` would typecheck —
// here, any `Settings<Yes, _, _>` (the required `endpoint` flag = `Yes`).
#[transitions]
impl<'a, B: SettingsReady + Send + 'a> Worker<'a, B> {
    /// Async deferred — Resolved → InFlight; lifts the chain into a
    /// pending future that resolves at the user's terminal `.await?`.
    #[transition(into = Running)]
    async fn start(state: B, ctx: &()) -> Result<Running, Error> {
        let s = state.finalize();
        println!(
            "starting workspace #{} against {}",
            s.workspace_id, s.endpoint
        );
        Ok(Running { id: s.workspace_id })
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let running = Worker(Pipeline::resolved(&(), Idle))
        .configure(7) //                                  sync infallible       -> Resolved
        .endpoint("https://api.example".into()) //        async-fallible setter -> lifts to InFlight
        .with_timeout_secs(60) //                         factory setter        -> folds into pending
        .with_label("nightly".into()) //                  factory setter        -> folds into pending
        .start() //                                       async deferred        -> folds into pending
        .await?; //                                                             -> Resolved

    println!("started: workspace #{}", running.0.into_state().id);
    Ok(())
}
