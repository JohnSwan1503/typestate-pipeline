//! Raw-style (EvmRpc) manifest building.
//!
//! Where the derived sub-pipeline collapses to a single sub-phase, the raw
//! sub-pipeline tracks several distinct manifest fields with different
//! requirements: `network` is mandatory; `finalized_blocks_only`,
//! `start_block`, and `tables` are optional with declared defaults. This
//! is exactly the shape `TypestateFactory` was built for — declare the
//! fields once with `#[field(...)]` annotations and the macro emits the
//! per-field flag generics, the bag struct, the standalone setters, and —
//! thanks to `pipeline(carrier = Author)` — the matching Resolved + InFlight
//! arms on the [`Author`] carrier. No phase-marker plumbing required.

use serde_json::value::RawValue;

use crate::TypestateFactory;

use super::{
    error::AuthoringError,
    kinds::{EvmRpc, Kind},
    phase::Registered,
    primitives::{Name, Namespace, NetworkId, Reference, TableName},
    Author,
};

// ---------------------------------------------------------------------------
// Per-kind "selected" state — populated by `new_evm_rpc` / `edit_existing_evm_rpc`.
// ---------------------------------------------------------------------------

/// State after the kind has been chosen and the dataset's identity is fixed,
/// but no manifest fields have been populated yet.
#[derive(Debug, Clone)]
pub struct EvmRpcSelected {
    pub namespace: Namespace,
    pub name: Name,
}

// ---------------------------------------------------------------------------
// Manifest builder — TypestateFactory bag with pipeline integration.
// ---------------------------------------------------------------------------

/// The raw manifest payload. Once every required flag is `Yes` and every
/// optional flag has either been set or has a default, the bag can be
/// finalized into a [`RawManifestData`] for serialization.
///
/// `pipeline(carrier = Author)` instructs `TypestateFactory` to *also* emit
/// every standalone arm (setters, default helpers) as a Resolved + InFlight
/// pair on `Author<'a, RawManifest<…>, M>`. That single attribute is what
/// turns this bag into a ten-method pipeline surface — without it we'd be
/// hand-rolling each `with_*` arm twice (once for each mode).
#[derive(Debug, TypestateFactory)]
#[factory(name = RawManifest, pipeline(carrier = Author))]
pub struct RawManifestData {
    /// Dataset namespace — populated by `into_builder` before the bag is
    /// handed to the user, so the visible setter is never called externally.
    /// Kept in the bag so `register` has everything it needs in one place.
    #[field(required)]
    pub namespace: Namespace,

    /// Dataset name — same story as `namespace`.
    #[field(required)]
    pub name: Name,

    /// Chain network this RPC source ingests from. Required; `overridable`
    /// so the existing-dataset edit path can swap it without touching the
    /// other already-set fields.
    #[field(required, name = with_network, overridable)]
    pub network: NetworkId,

    /// If `true`, only finalized blocks are ingested. Default `false`.
    #[field(default = false)]
    pub finalized_blocks_only: bool,

    /// First block to ingest from. Default `0`.
    #[field(default = 0)]
    pub start_block: u64,

    /// Table-name list. Defaults to an empty Vec; the
    /// `with_default_tables()` helper (named via `default_helper = …`)
    /// applies that default and flips the flag to `Yes`, matching the
    /// upstream amp call-site spelling.
    #[field(default = Vec::new(), default_helper = with_default_tables)]
    pub tables: Vec<TableName>,
}

// ---------------------------------------------------------------------------
// Transitions
// ---------------------------------------------------------------------------

use crate::{transitions, No, Yes};

#[transitions]
impl<'a> Author<'a, EvmRpcSelected> {
    /// Enter the manifest builder. Sync — assembles a fresh bag with
    /// `namespace` + `name` already set (their flags = `Yes`) and every
    /// other flag = `No`.
    #[transition(into = RawManifest<Yes, Yes, No, No, No, No>)]
    pub fn into_builder(state: EvmRpcSelected) -> RawManifest<Yes, Yes, No, No, No, No> {
        RawManifest::new().namespace(state.namespace).name(state.name)
    }
}

// All the field-level transitions — `with_network`, `with_finalized_blocks_only`,
// `with_start_block`, `tables_default`, `override_network`, etc. — are
// generated automatically by the `pipeline(carrier = Author)` arm of the
// `TypestateFactory` derive above. Only the cross-bag terminal `register`
// needs an explicit transition impl.

#[transitions]
impl<'a> Author<'a, RawManifest<Yes, Yes, Yes, Yes, Yes, Yes>> {
    /// Serialize the bag and POST to the server. Async — joins the raw
    /// sub-pipeline back into the main pipeline at [`Registered`].
    ///
    /// The bound on the impl block (`<Yes, Yes, Yes, Yes, Yes, Yes>`) is
    /// what makes `register` only callable when *every* manifest flag has
    /// been satisfied — required by setter, optional by default helper.
    /// The compiler enforces the precondition automatically.
    #[transition(into = Registered)]
    pub async fn register(
        state: RawManifest<Yes, Yes, Yes, Yes, Yes, Yes>,
        ctx: &super::client::Client,
    ) -> Result<Registered, AuthoringError> {
        let data = state.finalize();
        let body = serialize_raw(&data)?;
        let manifest_hash = ctx
            .register(data.namespace.clone(), data.name.clone(), body, EvmRpc::TAG)
            .await;
        Ok(Registered {
            namespace: data.namespace,
            name: data.name,
            manifest_hash,
        })
    }
}

// ---------------------------------------------------------------------------
// `edit_existing_evm_rpc` resolved-arm helper.
// ---------------------------------------------------------------------------

pub(crate) async fn edit_existing(
    client: &super::client::Client,
    reference: &Reference,
) -> Result<EvmRpcSelected, AuthoringError> {
    let (kind, _body) = client
        .fetch_manifest(&reference.namespace, &reference.name)
        .await
        .ok_or_else(|| AuthoringError::NotFound {
            namespace: reference.namespace.clone(),
            name: reference.name.clone(),
        })?;
    if kind != EvmRpc::TAG {
        return Err(AuthoringError::KindMismatch {
            expected: EvmRpc::TAG,
            actual: kind,
        });
    }
    Ok(EvmRpcSelected {
        namespace: reference.namespace.clone(),
        name: reference.name.clone(),
    })
}

// ---------------------------------------------------------------------------
// Manifest serialization.
// ---------------------------------------------------------------------------

fn serialize_raw(data: &RawManifestData) -> Result<Box<RawValue>, AuthoringError> {
    let body = serde_json::json!({
        "kind": EvmRpc::TAG,
        "network": data.network,
        "finalized_blocks_only": data.finalized_blocks_only,
        "start_block": data.start_block,
        "tables": data.tables,
    });
    let s = serde_json::to_string(&body).map_err(AuthoringError::Serialize)?;
    RawValue::from_string(s).map_err(AuthoringError::Parse)
}
