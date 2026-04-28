//! Integration test exercising all four transition shapes.
//!
//! Models a slice of the amp `client-admin::author` pipeline:
//!     Registered → Versioned → JobConfigured → Deployed
//! covering every body shape the macro must handle:
//!   - sync infallible:   `with_parallelism` (Versioned → JobConfigured)
//!   - sync fallible:     `validate_and_finalize` (JobConfigured → JobConfigured)
//!   - async deferred:    `tag_version` (Registered → Versioned), `deploy`
//!   - async breakpoint:  `confirm_and_tag` (Registered → Versioned)

use std::{
    fmt,
    sync::atomic::{AtomicU64, Ordering},
};

use typestate_pipeline::{pipelined, transitions, Pipeline, Resolved};

// ---------------------------------------------------------------------------
// Mock context — counts admin-side mutations so tests can assert ordering.
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
struct Client {
    next_job_id: AtomicU64,
}

impl Client {
    fn allocate_job_id(&self) -> u64 {
        self.next_job_id.fetch_add(1, Ordering::SeqCst) + 1
    }
}

#[derive(Debug)]
enum TestError {
    Invalid(&'static str),
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestError::Invalid(m) => write!(f, "invalid: {m}"),
        }
    }
}

impl std::error::Error for TestError {}

// ---------------------------------------------------------------------------
// Phase states
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Registered {
    name: String,
    manifest_hash: u64,
}

#[derive(Debug, Clone)]
struct Versioned {
    name: String,
    manifest_hash: u64,
    version: u32,
}

#[derive(Debug, Clone)]
struct JobConfigured {
    name: String,
    version: u32,
    parallelism: u16,
    verified: bool,
}

#[derive(Debug, Clone)]
struct Deployed {
    name: String,
    version: u32,
    job_id: u64,
}

// ---------------------------------------------------------------------------
// Newtype carrier wrapping `Pipeline`. Inherent impls on a foreign type via
// type alias are forbidden by the orphan rules — the wrapper makes the carrier
// a local type so `#[transitions]` can emit inherent impl blocks.
// ---------------------------------------------------------------------------

pipelined!(Author, ctx = Client, error = TestError);

impl<'a> Author<'a, Registered> {
    fn from_registered(client: &'a Client, name: &str, manifest_hash: u64) -> Self {
        Author(Pipeline::resolved(
            client,
            Registered {
                name: name.to_owned(),
                manifest_hash,
            },
        ))
    }
}

// Convenience accessor used in assertions.
impl<'a, S: 'a> Author<'a, S, Resolved> {
    fn state(&self) -> &S {
        self.0.state()
    }
}

// ---------------------------------------------------------------------------
// Registered → Versioned: async deferred + async breakpoint
// ---------------------------------------------------------------------------

#[transitions(error = TestError)]
impl<'a> Author<'a, Registered> {
    /// Async deferred — folds into the chain.
    #[transition(into = Versioned)]
    pub async fn tag_version(
        state: Registered,
        ctx: &Client,
        version: u32,
    ) -> Result<Versioned, TestError> {
        let _ = ctx;
        if version == 0 {
            return Err(TestError::Invalid("version must be > 0"));
        }
        Ok(Versioned {
            name: state.name,
            manifest_hash: state.manifest_hash,
            version,
        })
    }

    /// Async breakpoint — caller must `.await?` here.
    #[transition(into = Versioned, deferred = false)]
    pub async fn confirm_and_tag(
        state: Registered,
        ctx: &Client,
    ) -> Result<Versioned, TestError> {
        let _ = ctx;
        Ok(Versioned {
            name: state.name,
            manifest_hash: state.manifest_hash,
            version: 1,
        })
    }
}

// ---------------------------------------------------------------------------
// Versioned → JobConfigured: sync infallible
// ---------------------------------------------------------------------------

#[transitions(error = TestError)]
impl<'a> Author<'a, Versioned> {
    #[transition(into = JobConfigured)]
    pub fn with_parallelism(state: Versioned, parallelism: u16) -> JobConfigured {
        JobConfigured {
            name: state.name,
            version: state.version,
            parallelism,
            verified: false,
        }
    }
}

// ---------------------------------------------------------------------------
// JobConfigured → JobConfigured: sync fallible (in-place validation)
// JobConfigured → Deployed: async deferred
// ---------------------------------------------------------------------------

#[transitions(error = TestError)]
impl<'a> Author<'a, JobConfigured> {
    /// Sync fallible — Resolved arm returns `Result<…, TestError>` directly,
    /// InFlight arm folds the Result into the chained pending.
    #[transition(into = JobConfigured)]
    pub fn validate_and_finalize(
        mut state: JobConfigured,
    ) -> Result<JobConfigured, TestError> {
        if state.parallelism == 0 {
            return Err(TestError::Invalid("parallelism must be > 0"));
        }
        state.verified = true;
        Ok(state)
    }

    #[transition(into = Deployed)]
    pub async fn deploy(state: JobConfigured, ctx: &Client) -> Result<Deployed, TestError> {
        if !state.verified {
            return Err(TestError::Invalid("not verified before deploy"));
        }
        let job_id = ctx.allocate_job_id();
        Ok(Deployed {
            name: state.name,
            version: state.version,
            job_id,
        })
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn full_chain_with_resolved_breakpoint_in_middle() {
    let client = Client::default();
    let pipeline = Author::from_registered(&client, "ds-a", 0xCAFE);

    // Resolved-direct: tag_version returns InFlight, but deferred=true folds
    // into the rest of the chain — single terminal `.await?`.
    let deployed: Author<Deployed> = pipeline
        .tag_version(7)
        .with_parallelism(8)
        .validate_and_finalize() // sync fallible — folds Result into pending
        .deploy()
        .await
        .expect("chain failed");

    let state = deployed.state().clone();
    assert_eq!(state.name, "ds-a");
    assert_eq!(state.version, 7);
    assert_eq!(state.job_id, 1);
}

#[tokio::test]
async fn breakpoint_forces_explicit_await() {
    let client = Client::default();
    let pipeline = Author::from_registered(&client, "ds-b", 0xBEEF);

    // confirm_and_tag is `deferred = false` — its return type is
    // `Result<Author<Versioned, Resolved>, TestError>`, so the chain breaks
    // here and the next link is invoked on a Resolved pipeline.
    let versioned: Author<Versioned, Resolved> = pipeline
        .confirm_and_tag()
        .await
        .expect("breakpoint await failed");

    // After the breakpoint we're back in Resolved mode. Sync-fallible
    // `validate_and_finalize` correctly returns a `Result` and breaks the
    // chain — the user must handle it before chaining `.deploy()`.
    let configured = versioned
        .with_parallelism(4)
        .validate_and_finalize()
        .expect("validation failed");
    let deployed = configured.deploy().await.expect("deploy failed");

    assert_eq!(deployed.state().version, 1);
    assert_eq!(deployed.state().job_id, 1);
}

#[tokio::test]
async fn sync_fallible_resolved_returns_result_directly() {
    let client = Client::default();
    let pipeline = Author::from_registered(&client, "ds-c", 1)
        .confirm_and_tag()
        .await
        .unwrap();

    // Resolved → Resolved: sync fallible returns `Result<…, TestError>` on the
    // Resolved arm, no future indirection.
    let configured: Author<JobConfigured, Resolved> =
        pipeline.with_parallelism(2).validate_and_finalize().unwrap();

    assert!(configured.state().verified);
    assert_eq!(configured.state().parallelism, 2);
}

#[tokio::test]
async fn sync_fallible_propagates_through_inflight_chain() {
    let client = Client::default();
    let pipeline = Author::from_registered(&client, "ds-d", 1);

    // parallelism = 0 fails inside validate_and_finalize; the error must
    // bubble out through the chained pending future.
    let result = pipeline
        .tag_version(1)
        .with_parallelism(0)
        .validate_and_finalize()
        .deploy()
        .await;

    match result {
        Err(TestError::Invalid(msg)) => assert_eq!(msg, "parallelism must be > 0"),
        Ok(_) => panic!("expected validation error"),
    }
}

#[tokio::test]
async fn intofuture_resolves_inflight_back_to_resolved() {
    let client = Client::default();
    let pipeline = Author::from_registered(&client, "ds-e", 9).tag_version(3); // InFlight

    // Awaiting an InFlight pipeline directly hands back a Resolved one.
    let versioned: Author<Versioned, Resolved> = pipeline.await.unwrap();
    assert_eq!(versioned.state().version, 3);
    assert_eq!(versioned.state().manifest_hash, 9);
}
