#![allow(unused)]

#[path = "attr_forwarding/tests/impl_attr_forwarded_to_both_arms.rs"]
pub mod impl_attr_forwarded_to_both_arms;

// ---------------------------------------------------------------------------
// Impl-level attributes reach both Resolved and InFlight arms.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn impl_attr_forwarded_to_both_arms() {
    impl_attr_forwarded_to_both_arms::main().await;
}
