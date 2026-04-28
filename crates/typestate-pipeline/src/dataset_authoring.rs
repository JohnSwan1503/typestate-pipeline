//! End-to-end dataset-authoring pipeline rebuilt on top of `typestate-pipeline`.
//! (Built by Claude.. be nice)
//!
//! Demonstrates the three macros in concert:
//!
//! - [`pipelined!`](crate::pipelined) declares the [`Author`] carrier —
//!   the dual-mode struct, its `Pipelined<'a>` impl, and `IntoFuture`
//!   forwarding in one line.
//! - [`#[transitions]`](crate::transitions) generates Resolved + InFlight
//!   method pairs from each cross-phase method body.
//! - [`#[derive(TypestateFactory)]`](crate::TypestateFactory) with
//!   `pipeline(carrier = Author)` declares the deploy-parameters bag
//!   (`JobConfig`) and emits both the standalone setters and the
//!   pipeline arms on [`Author`].

// `'a` on `&'a Reference` in the `edit_existing_*` transitions is needed
// for the generated InFlight future to capture the borrow. Clippy's
// `needless_lifetimes` still fires against the original token spans even
// after `#[allow]` is forwarded by `#[transitions]`, so suppress at the
// module level.
#![allow(clippy::needless_lifetimes)]

pub mod client;
pub mod derived;
pub mod error;
pub mod kinds;
pub mod phase;
pub mod primitives;
pub mod raw;

use client::Client;
use derived::DerivedSelected;
use error::AuthoringError;
use phase::{Deployed, JobConfig, JobConfigReady, Registered, Versioned};
use primitives::{Name, Namespace, Reference, Version};
use raw::EvmRpcSelected;

use crate::{Pipeline, Resolved, pipelined, transitions};

pipelined!(pub Author, ctx = Client, error = AuthoringError);

/// Empty initial state. The two `new_*` transitions consume it; the two
/// `edit_existing_*` transitions consume it asynchronously after fetching
/// the existing manifest and verifying its kind.
#[derive(Debug, Clone, Copy, Default)]
pub struct Init;

impl Client {
    /// Open an authoring pipeline against this client. Equivalent of
    /// `client.author()` in the upstream API.
    pub fn author(&self) -> Author<'_, Init, Resolved> {
        Author(Pipeline::resolved(self, Init))
    }
}

#[transitions]
impl<'a> Author<'a, Init> {
    /// Start authoring a fresh EVM-RPC dataset. Sync — just commits
    /// identity into the next phase.
    #[transition(into = EvmRpcSelected)]
    pub fn new_evm_rpc(state: Init, namespace: Namespace, name: Name) -> EvmRpcSelected {
        EvmRpcSelected { namespace, name }
    }

    /// Start authoring a fresh derived dataset. Sync.
    #[transition(into = DerivedSelected)]
    pub fn new_derived(state: Init, namespace: Namespace, name: Name) -> DerivedSelected {
        DerivedSelected { namespace, name }
    }

    /// Open the authoring pipeline against an existing EVM-RPC dataset.
    /// Async — fetches the current manifest, verifies the kind matches.
    #[transition(into = EvmRpcSelected)]
    pub async fn edit_existing_evm_rpc(
        state: Init,
        ctx: &Client,
        reference: &'a Reference,
    ) -> Result<EvmRpcSelected, AuthoringError> {
        raw::edit_existing(ctx, reference).await
    }

    /// Open the authoring pipeline against an existing derived dataset.
    /// Async — fetches the current manifest, verifies the kind matches.
    #[transition(into = DerivedSelected)]
    pub async fn edit_existing_derived(
        state: Init,
        ctx: &Client,
        reference: &'a Reference,
    ) -> Result<DerivedSelected, AuthoringError> {
        derived::edit_existing(ctx, reference).await
    }
}

// `Versioned` is the `JobConfig` bag with every deploy-param flag = `No`.
// The four head fields (namespace, name, hash, version) are passed
// positionally to `JobConfig::new(…)` — they're declared `#[field(internal)]`
// so they're locked in at construction and don't appear in the bag's
// flag-generic surface.
#[transitions]
impl<'a> Author<'a, Registered> {
    /// Tag the registered manifest with an explicit version. Async — POSTs
    /// to the server's tag endpoint, then advances to the
    /// version-but-not-yet-deploy-configured phase.
    #[transition(into = Versioned)]
    pub async fn tag_version(
        state: Registered,
        ctx: &Client,
        version: Version,
    ) -> Result<Versioned, AuthoringError> {
        ctx.tag(state.namespace.clone(), state.name.clone(), version)
            .await;
        Ok(JobConfig::new(
            state.namespace,
            state.name,
            state.manifest_hash,
            version,
        ))
    }

    /// Bump the patch component of the dataset's current `latest` tag and
    /// apply it. Async — fetch + tag in one transition. Errors if the
    /// dataset has no prior version.
    #[transition(into = Versioned)]
    pub async fn bump_patch(state: Registered, ctx: &Client) -> Result<Versioned, AuthoringError> {
        let latest = ctx
            .fetch_latest(&state.namespace, &state.name)
            .await
            .ok_or(AuthoringError::NoPriorVersion)?;
        let next = latest.next_patch();
        ctx.tag(state.namespace.clone(), state.name.clone(), next)
            .await;
        Ok(JobConfig::new(
            state.namespace,
            state.name,
            state.manifest_hash,
            next,
        ))
    }
}

// `with_verify` / `with_worker` / `with_parallelism` are emitted by
// `TypestateFactory`'s `pipeline(carrier = Author)` arm. The deploy gate
// is the auto-generated `JobConfigReady` trait bound — a single bound
// that abstracts over the full `<Yes, Yes, …, FlagN>` flag tuple.
#[transitions]
impl<'a, B> Author<'a, B>
where
    B: JobConfigReady + Send + 'a,
{
    /// Schedule the deploy job. Async terminal — only callable when every
    /// required flag (the four head fields plus `parallelism`) is `Yes`.
    /// Optional flags `verify`/`worker` may be either: their declared
    /// defaults apply at `into_finalized()` when unset.
    #[transition(into = Deployed)]
    pub async fn deploy(state: B, ctx: &Client) -> Result<Deployed, AuthoringError> {
        let data = state.into_finalized();
        let job_id = ctx.allocate_job_id().await;
        Ok(Deployed {
            namespace: data.namespace,
            name: data.name,
            manifest_hash: data.manifest_hash,
            version: data.version,
            parallelism: data.parallelism,
            job_id,
        })
    }
}

impl<'a> Author<'a, Deployed, Resolved> {
    pub fn job_id(&self) -> primitives::JobId {
        self.0.state().job_id
    }

    pub fn reference(&self) -> Reference {
        let st = self.0.state();
        Reference {
            namespace: st.namespace.clone(),
            name: st.name.clone(),
            version: st.version,
        }
    }
}
