use std::sync::atomic::{AtomicU64, Ordering};

use typestate_pipeline::{Pipeline, Resolved, pipelined, transitions};

use super::error::TestError;
use super::phases::{Deployed, JobConfigured, Registered, Versioned};

// ---------------------------------------------------------------------------
// Mock context — counts admin-side mutations so tests can assert
// ordering.
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Client {
    pub next_job_id: AtomicU64,
}

impl Client {
    pub fn allocate_job_id(&self) -> u64 {
        self.next_job_id.fetch_add(1, Ordering::SeqCst) + 1
    }
}

// ---------------------------------------------------------------------------
// Carrier + transitions covering all four body shapes.
// ---------------------------------------------------------------------------

pipelined!(pub Author, ctx = Client, error = TestError);

impl<'a> Author<'a, Registered> {
    pub fn from_registered(client: &'a Client, name: &str, manifest_hash: u64) -> Self {
        Author(Pipeline::resolved(
            client,
            Registered {
                name: name.to_owned(),
                manifest_hash,
            },
        ))
    }
}

impl<'a, S: 'a> Author<'a, S, Resolved> {
    pub fn state(&self) -> &S {
        self.0.state()
    }
}

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
    #[transition(into = Versioned, breakpoint)]
    pub async fn confirm_and_tag(state: Registered, ctx: &Client) -> Result<Versioned, TestError> {
        let _ = ctx;
        Ok(Versioned {
            name: state.name,
            manifest_hash: state.manifest_hash,
            version: 1,
        })
    }
}

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

#[transitions(error = TestError)]
impl<'a> Author<'a, JobConfigured> {
    /// Sync fallible — Resolved arm returns `Result<…, TestError>`
    /// directly, InFlight arm folds the Result into the chained
    /// pending.
    #[transition(into = JobConfigured)]
    pub fn validate_and_finalize(mut state: JobConfigured) -> Result<JobConfigured, TestError> {
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
