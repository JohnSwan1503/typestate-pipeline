//! Derived-dataset manifest building.
//!
//! In the upstream `amp` flow, the derived sub-pipeline collapses to a
//! single `Building` sub-phase because *no* manifest fields are required:
//! every collection (tables, dependencies, functions) starts empty and the
//! user appends entries before calling `register`. We mirror that shape here
//! with a plain struct + a `#[transitions]` impl that emits append-only
//! `add_*` transitions plus a terminal async `register`.
//!
//! TypestateFactory would be the wrong tool here: there are no required
//! fields to track at the type level. The `transitions` proc-macro alone
//! gives us the dual-mode (Resolved + InFlight) method pairs we need.

use serde_json::value::RawValue;

use crate::transitions;

use super::{
    Author,
    error::AuthoringError,
    kinds::{Derived, Kind},
    phase::Registered,
    primitives::{Name, Namespace, Reference, TableName},
};

// ---------------------------------------------------------------------------------
// Per-kind "selected" state — populated by `new_derived` / `edit_existing_derived`.
// ---------------------------------------------------------------------------------

/// State after the kind has been chosen and the dataset's identity is fixed,
/// but the manifest has not yet been built.
///
/// Two transitions originate here:
/// - `into_builder()` — sync, lands in [`DerivedBuilder`] with empty lists.
/// - the upstream pipeline can also be entered into via
///   [`edit_existing_derived`](super::Author::edit_existing_derived), which
///   ends up at this state in `InFlight` mode after fetching + verifying
///   the existing dataset.
#[derive(Debug, Clone)]
pub struct DerivedSelected {
    pub namespace: Namespace,
    pub name: Name,
}

// ---------------------------------------------------------------------------
// Builder: append-only manifest accumulator.
// ---------------------------------------------------------------------------

/// Builder accumulating the parts of a derived manifest. Carries the
/// dataset's identity so that `register` can submit the body to the server
/// without needing extra parameters.
#[derive(Debug, Clone)]
pub struct DerivedBuilder {
    pub namespace: Namespace,
    pub name: Name,
    pub tables: Vec<(TableName, String)>,
    pub dependencies: Vec<(String, String)>,
    pub functions: Vec<(String, String)>,
}

// ---------------------------------------------------------------------------
// Transitions
// ---------------------------------------------------------------------------

#[transitions]
impl<'a> Author<'a, DerivedSelected> {
    /// Enter the manifest builder. Sync — no I/O, just hands over the
    /// identity to the empty accumulator.
    #[transition(into = DerivedBuilder)]
    pub fn into_builder(state: DerivedSelected) -> DerivedBuilder {
        DerivedBuilder {
            namespace: state.namespace,
            name: state.name,
            tables: Vec::new(),
            dependencies: Vec::new(),
            functions: Vec::new(),
        }
    }
}

#[transitions]
impl<'a> Author<'a, DerivedBuilder> {
    /// Append a SQL-defined table. Stays in [`DerivedBuilder`].
    #[transition(into = DerivedBuilder)]
    pub fn add_table(mut state: DerivedBuilder, name: TableName, sql: String) -> DerivedBuilder {
        state.tables.push((name, sql));
        state
    }

    /// Append an external-dataset dependency. Stays in [`DerivedBuilder`].
    #[transition(into = DerivedBuilder)]
    pub fn add_dependency(mut state: DerivedBuilder, alias: String, dep: String) -> DerivedBuilder {
        state.dependencies.push((alias, dep));
        state
    }

    /// Append a user-defined function. Stays in [`DerivedBuilder`].
    #[transition(into = DerivedBuilder)]
    pub fn add_function(mut state: DerivedBuilder, name: String, body: String) -> DerivedBuilder {
        state.functions.push((name, body));
        state
    }

    /// Serialize the manifest and POST it to the server. Async — joins the
    /// derived sub-pipeline back into the main pipeline at [`Registered`].
    #[transition(into = Registered)]
    pub async fn register(
        state: DerivedBuilder,
        ctx: &super::client::Client,
    ) -> Result<Registered, AuthoringError> {
        let body = serialize_derived(&state)?;
        let manifest_hash = ctx
            .register(
                state.namespace.clone(),
                state.name.clone(),
                body,
                Derived::TAG,
            )
            .await;
        Ok(Registered {
            namespace: state.namespace,
            name: state.name,
            manifest_hash,
        })
    }
}

// ---------------------------------------------------------------------------
// `edit_existing_derived` resolved-arm helper.
// ---------------------------------------------------------------------------

/// Async fetch + kind-check used by [`Author::edit_existing_derived`]. Lives
/// here (rather than in `dataset_authoring.rs`) so the per-kind logic is
/// co-located with the rest of the derived path.
pub(crate) async fn edit_existing(
    client: &super::client::Client,
    reference: &Reference,
) -> Result<DerivedSelected, AuthoringError> {
    let (kind, _body) = client
        .fetch_manifest(&reference.namespace, &reference.name)
        .await
        .ok_or_else(|| AuthoringError::NotFound {
            namespace: reference.namespace.clone(),
            name: reference.name.clone(),
        })?;
    if kind != Derived::TAG {
        return Err(AuthoringError::KindMismatch {
            expected: Derived::TAG,
            actual: kind,
        });
    }
    Ok(DerivedSelected {
        namespace: reference.namespace.clone(),
        name: reference.name.clone(),
    })
}

// ---------------------------------------------------------------------------
// Manifest serialization.
// ---------------------------------------------------------------------------

fn serialize_derived(state: &DerivedBuilder) -> Result<Box<RawValue>, AuthoringError> {
    let body = serde_json::json!({
        "kind": Derived::TAG,
        "tables": state.tables.iter().map(|(n, sql)| {
            serde_json::json!({ "name": n, "sql": sql })
        }).collect::<Vec<_>>(),
        "dependencies": state.dependencies.iter().map(|(a, d)| {
            serde_json::json!({ "alias": a, "ref": d })
        }).collect::<Vec<_>>(),
        "functions": state.functions.iter().map(|(n, b)| {
            serde_json::json!({ "name": n, "body": b })
        }).collect::<Vec<_>>(),
    });
    let s = serde_json::to_string(&body).map_err(AuthoringError::Serialize)?;
    RawValue::from_string(s).map_err(AuthoringError::Parse)
}
