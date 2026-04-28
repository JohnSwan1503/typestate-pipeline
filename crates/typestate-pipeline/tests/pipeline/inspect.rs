//! Verify the `inspect` combinator emitted by `pipelined!` /
//! `impl_pipelined!`.
//!
//! Two arms tested:
//!
//! - **Resolved.** Closure runs synchronously on the carrier; carrier is
//!   returned unchanged so the chain continues.
//! - **InFlight (deferred).** Closure runs *after* the pending future
//!   resolves, against a temporary `Resolved` carrier reference. The
//!   chain returns to `InFlight` so subsequent transitions keep folding
//!   into a single terminal `.await?`.

use std::{
    cell::RefCell,
    fmt,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};

use typestate_pipeline::{pipelined, transitions, Pipeline, Resolved};

#[derive(Debug, Default)]
struct Hub {
    next_id: AtomicU64,
}

impl Hub {
    fn allocate(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::SeqCst) + 1
    }
}

#[derive(Debug)]
enum AppError {}
impl fmt::Display for AppError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
impl std::error::Error for AppError {}

pipelined!(Author, ctx = Hub, error = AppError);

#[derive(Debug, Clone)]
struct Drafted {
    name: String,
}

#[derive(Debug, Clone)]
struct Tagged {
    name: String,
    tag: u64,
}

#[derive(Debug, Clone)]
struct Deployed {
    name: String,
    tag: u64,
    job_id: u64,
}

#[transitions]
impl<'a> Author<'a, Drafted> {
    /// Async deferred — produces an InFlight chain.
    #[transition(into = Tagged)]
    pub async fn tag(state: Drafted, ctx: &Hub) -> Result<Tagged, AppError> {
        Ok(Tagged {
            name: state.name,
            tag: ctx.allocate(),
        })
    }
}

#[transitions]
impl<'a> Author<'a, Tagged> {
    #[transition(into = Deployed)]
    pub async fn deploy(state: Tagged, ctx: &Hub) -> Result<Deployed, AppError> {
        Ok(Deployed {
            name: state.name,
            tag: state.tag,
            job_id: ctx.allocate(),
        })
    }
}

// ---------------------------------------------------------------------------
// Resolved arm
// ---------------------------------------------------------------------------

#[tokio::test]
async fn resolved_inspect_runs_sync_and_preserves_chain() {
    let hub = Hub::default();
    let pipeline = Author(Pipeline::resolved(
        &hub,
        Drafted {
            name: "alpha".to_owned(),
        },
    ));

    // Resolved → Resolved: closure must run synchronously and observe
    // the carrier's current state.
    let observed = RefCell::new(None::<String>);
    let _: Author<Drafted, Resolved> = pipeline.inspect(|c| {
        *observed.borrow_mut() = Some(c.0.state().name.clone());
    });

    assert_eq!(observed.borrow().as_deref(), Some("alpha"));
}

#[tokio::test]
async fn resolved_inspect_does_not_change_typestate() {
    // The combinator returns `Self` unchanged, so a Resolved-mode
    // inspect inside a chain doesn't disrupt the downstream type. We
    // witness this by chaining `.tag().await?` after the inspect: the
    // type system has to accept `inspect(...)` as still-Drafted for
    // `tag` to be callable on it.
    let hub = Hub::default();
    let pipeline = Author(Pipeline::resolved(
        &hub,
        Drafted {
            name: "beta".to_owned(),
        },
    ));

    let tagged = pipeline
        .inspect(|c| {
            assert_eq!(c.0.state().name, "beta");
        })
        .tag()
        .await
        .unwrap();
    let resolved_tagged: Author<Tagged, Resolved> = tagged;
    assert_eq!(resolved_tagged.0.state().name, "beta");
    assert_eq!(resolved_tagged.0.state().tag, 1);
}

// ---------------------------------------------------------------------------
// InFlight arm (deferred)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn inflight_inspect_runs_after_pending_resolves() {
    // Set up a pipeline that's about to enter the chain. `tag()` is
    // async-deferred, so the result is `Author<Tagged, InFlight>`.
    // Calling `.inspect(...)` on InFlight defers the closure until the
    // future is awaited.
    let hub = Hub::default();
    let pipeline = Author(Pipeline::resolved(
        &hub,
        Drafted {
            name: "gamma".to_owned(),
        },
    ));

    // Use a Mutex to record observations from the closure (it runs
    // inside an `async move` and needs `Send`).
    let log = Arc::new(Mutex::new(Vec::<String>::new()));
    let log_for_inspect = Arc::clone(&log);

    // Before-await: the closure must NOT have run yet.
    let chain = pipeline.tag().inspect(move |c| {
        log_for_inspect
            .lock()
            .unwrap()
            .push(format!("inspected name={} tag={}", c.0.state().name, c.0.state().tag));
    });
    assert!(
        log.lock().unwrap().is_empty(),
        "deferred inspect should NOT run until the chain is awaited"
    );

    // After-await: the closure has run, observed the resolved Tagged
    // state, and the chain has resolved to a Resolved carrier whose
    // state matches what the closure saw.
    let resolved: Author<Tagged, Resolved> = chain.await.unwrap();
    assert_eq!(*log.lock().unwrap(), vec!["inspected name=gamma tag=1"]);
    assert_eq!(resolved.0.state().name, "gamma");
    assert_eq!(resolved.0.state().tag, 1);
}

#[tokio::test]
async fn inflight_inspect_chains_through_subsequent_transitions() {
    // The InFlight `inspect` returns InFlight, so the chain should
    // continue into `deploy()` — itself an async-deferred transition —
    // and fold everything into one terminal `.await?`.
    let hub = Hub::default();
    let pipeline = Author(Pipeline::resolved(
        &hub,
        Drafted {
            name: "delta".to_owned(),
        },
    ));

    let log = Arc::new(Mutex::new(Vec::<String>::new()));
    let log_clone = Arc::clone(&log);

    let deployed = pipeline
        .tag()
        .inspect(move |c| {
            log_clone.lock().unwrap().push(format!("tag={}", c.0.state().tag));
        })
        .deploy()
        .await
        .unwrap();

    let log = log.lock().unwrap();
    assert_eq!(*log, vec!["tag=1"]);
    let state = deployed.0.into_state();
    assert_eq!(state.name, "delta");
    assert_eq!(state.tag, 1);
    assert_eq!(state.job_id, 2);
}
