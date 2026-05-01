//! Phase types for the main pipeline.
//!
//! These are the *post-builder* phases — the ones the two manifest-building
//! sub-pipelines (raw + derived) converge into. The earlier
//! `KindSelected`/`ManifestAttached` phases of the upstream `amp` flow are
//! collapsed here: the per-kind `*Selected` types in
//! [`dataset_authoring`](super) carry the same identity payload but live as
//! sibling state types instead of `<K>`-generic ones, since `pipelined!`'s
//! conventional carrier shape is `<'a, S, M>` with no kind axis.
//!
//! `JobConfig` is a `TypestateFactory` bag: each manifest-identity field
//! and each deploy parameter sits on its own type-level flag axis. The
//! `Versioned` alias and the `deploy` gate downstream are then expressed
//! purely as constraints on those flag generics — no hand-rolled
//! single-axis `Satisfiable` parameter, no `retag` helper, no
//! `from_registered` constructor.

use crate::TypestateFactory;
use typestate_pipeline_core::No;

use super::{
    Author,
    primitives::{Hash, JobId, Name, Namespace, Version},
};

// ---------------------------------------------------------------------------
// Registered — single rejoin point for the two sub-pipelines.
// ---------------------------------------------------------------------------

/// Phase reached after a manifest has been registered against the server.
///
/// This is the single rejoin point shared by both the raw (EvmRpc) and
/// derived sub-pipelines — past this point the kind no longer matters
/// for downstream transitions, since the server has already accepted the
/// kind-specific payload and handed back a content hash.
#[derive(Debug, Clone)]
pub struct Registered {
    pub namespace: Namespace,
    pub name: Name,
    pub manifest_hash: Hash,
}

// ---------------------------------------------------------------------------
// JobConfig — TypestateFactory bag for the deploy-config phase.
// ---------------------------------------------------------------------------

/// Job-configuration phase: a manifest has been registered and tagged with
/// a version, and we are now collecting deploy parameters.
///
/// Modeled as a `TypestateFactory` bag: each field has its own type-level
/// flag, and `pipeline(carrier = Author)` auto-emits the Resolved + InFlight
/// pipeline arms for each setter and default helper. The `deploy` gate
/// downstream is then expressed as a constraint on the bag's flag tuple —
/// see [`super::Author::deploy`].
///
/// Field roles:
///
/// - `namespace` / `name` / `manifest_hash` / `version` — required, set
///   exactly once by `tag_version`/`bump_patch` from the prior `Registered`
///   state. The bag's "all four head flags = `Yes`" shape *is* the
///   just-tagged phase; see [`Versioned`].
/// - `parallelism` — required (with the `with_parallelism` setter name),
///   so the bag cannot be finalized — and `deploy` therefore cannot be
///   reached — until the caller supplies a value.
/// - `verify` / `worker` — optional with declared defaults, so the bag
///   may be finalized whether or not the caller set them. When they are
///   left unset, the defaults below apply at finalize time.
#[derive(Debug, TypestateFactory)]
#[factory(name = JobConfig, pipeline(carrier = Author))]
pub struct JobConfiguredData {
    // The four head fields are populated by `tag_version` / `bump_patch`
    // before the bag is handed to the user. `internal` keeps the standalone
    // setters (the transition bodies need them) but skips the carrier-side
    // arms — `Author<'a, JobConfig<…>>::namespace(…)` would never be a
    // legitimate call, so it shouldn't pollute the public surface.
    #[field(required, internal)]
    pub namespace: Namespace,

    #[field(required, internal)]
    pub name: Name,

    #[field(required, internal)]
    pub manifest_hash: Hash,

    #[field(required, internal)]
    pub version: Version,

    /// Required so `finalize()` (and therefore [`deploy`](super::Author::deploy))
    /// can't be reached until parallelism is supplied. The `name = ` override
    /// preserves the call-site spelling — required fields default to a bare
    /// `<field>` setter, but every other deploy-param uses the `with_<field>`
    /// convention so we keep parallelism in line.
    #[field(required, name = with_parallelism)]
    pub parallelism: u16,

    #[field(default = false)]
    pub verify: bool,

    /// Optional worker pinning. Stored as `Option<Name>` so the unset
    /// state is representable, but `input = Name + setter = wrap_some`
    /// lets the call site read `.with_worker(name)` instead of
    /// `.with_worker(Some(name))`.
    #[field(default = None, setter = wrap_some, input = Name)]
    pub worker: Option<Name>,
}

/// Setter transformer for [`JobConfiguredData::worker`]. Lifts `Name` into
/// the `Option<Name>` storage shape so callers don't need to wrap.
fn wrap_some(name: Name) -> Option<Name> {
    Some(name)
}

/// Just-tagged shape: every deploy-param flag still `No`. The four head
/// fields (namespace / name / manifest_hash / version) are
/// `#[field(internal)]` so they have no flag generic — their values are
/// always present, supplied positionally to `JobConfig::new(…)` by the
/// transition body. The natural return type of
/// [`tag_version`](super::Author::tag_version) and
/// [`bump_patch`](super::Author::bump_patch).
pub type Versioned = JobConfig<No, No, No>;

// ---------------------------------------------------------------------------
// Deployed — terminal snapshot.
// ---------------------------------------------------------------------------

/// Terminal phase: the deploy job has been scheduled.
///
/// Carries the final `JobId` plus the data the user is most likely to want
/// for follow-on calls (e.g. polling the job status, building a
/// [`Reference`](super::primitives::Reference) to the new version).
#[derive(Debug, Clone)]
pub struct Deployed {
    pub namespace: Namespace,
    pub name: Name,
    pub manifest_hash: Hash,
    pub version: Version,
    pub parallelism: u16,
    pub job_id: JobId,
}
