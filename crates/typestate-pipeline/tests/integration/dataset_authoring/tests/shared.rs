pub use typestate_pipeline::dataset_authoring::{
    client::Client,
    primitives::{Name, Namespace, NetworkId, Reference, TableName, Version},
};

/// Build a `Namespace` from a string slice — saves a lot of `.to_owned()`
/// at the call sites.
pub fn ns(s: &str) -> Namespace {
    Namespace(s.to_owned())
}

/// Same shape as [`ns`] for the `Name` primitive.
pub fn nm(s: &str) -> Name {
    Name(s.to_owned())
}
