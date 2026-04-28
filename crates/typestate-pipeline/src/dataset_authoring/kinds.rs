//! Kind markers for the simplified dataset-authoring flow.
//!
//! In `amp`'s real client, the `Kind` axis is one of seven enum-like marker
//! types (`EvmRpc`, `BitcoinRpc`, `Solana`, `Firehose`, `Phaser`, `Static`,
//! `Derived`, …). Here we only model the two used in the example flows —
//! [`EvmRpc`] (a *raw*-style chain ingestion source) and [`Derived`] (a
//! SQL-defined dataset built on top of others).

/// Marker for an EVM-RPC ingestion source — a "raw" kind that requires a
/// `network` and a set of tables to be authored.
#[derive(Debug, Clone, Copy)]
pub struct EvmRpc;

/// Marker for a derived dataset — a SQL-defined dataset that depends on
/// other datasets and exposes computed tables.
#[derive(Debug, Clone, Copy)]
pub struct Derived;

/// Wire-tag string for a kind. Stored on the registered manifest so the
/// server can route the payload to the right validator. Used by
/// [`edit_existing_*`](super::Author) to verify the kind on the existing
/// dataset matches what the caller asked for.
pub trait Kind {
    const TAG: &'static str;
}

impl Kind for EvmRpc {
    const TAG: &'static str = "evm_rpc";
}

impl Kind for Derived {
    const TAG: &'static str = "derived";
}
