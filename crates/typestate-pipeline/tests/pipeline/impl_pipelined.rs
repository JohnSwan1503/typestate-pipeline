#![allow(unused)]

#[path = "impl_pipelined/tests/shared.rs"]
mod shared;

pub(self) use shared::*;

#[path = "impl_pipelined/tests/pipelined_associated_types_resolve.rs"]
pub mod pipelined_associated_types_resolve;

#[path = "impl_pipelined/tests/gat_projections_are_correct.rs"]
pub mod gat_projections_are_correct;

#[path = "impl_pipelined/tests/intofuture_drives_inflight_back_to_resolved.rs"]
pub mod intofuture_drives_inflight_back_to_resolved;

#[path = "impl_pipelined/tests/tagged_pipelined_resolves.rs"]
pub mod tagged_pipelined_resolves;

// ---------------------------------------------------------------------------
// Pipelined associated types resolve to declared shapes.
// ---------------------------------------------------------------------------

#[test]
fn pipelined_associated_types_resolve() {
    pipelined_associated_types_resolve::main();
}

// ---------------------------------------------------------------------------
// GAT projections produce the expected concrete carrier types.
// ---------------------------------------------------------------------------

#[test]
fn gat_projections_are_correct() {
    gat_projections_are_correct::main();
}

// ---------------------------------------------------------------------------
// IntoFuture drives an InFlight carrier back to its Resolved successor.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn intofuture_drives_inflight_back_to_resolved() {
    intofuture_drives_inflight_back_to_resolved::main().await;
}

// ---------------------------------------------------------------------------
// Tagged carrier satisfies Pipelined<'a, Tag = MyTag>.
// ---------------------------------------------------------------------------

#[test]
fn tagged_pipelined_resolves() {
    tagged_pipelined_resolves::main();
}
