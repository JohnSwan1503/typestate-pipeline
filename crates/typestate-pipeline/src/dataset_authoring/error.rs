use thiserror::Error;

use crate::dataset_authoring::primitives::{Name, Namespace};

#[derive(Debug, Error)]
pub enum AuthoringError {
    #[error("dataset not found: {namespace}/{name}")]
    NotFound { namespace: Namespace, name: Name },

    #[error("kind mismatch: expected {expected}, server returned {actual}")]
    KindMismatch {
        expected: &'static str,
        actual: String,
    },

    #[error("manifest serialization failed")]
    Serialize(#[source] serde_json::Error),

    #[error("manifest parse failed")]
    Parse(#[source] serde_json::Error),

    #[error("missing required manifest field `{0}`")]
    MissingField(&'static str),

    #[error("cannot bump version: dataset has no `latest` tag")]
    NoPriorVersion,

    #[error("validation failed: {0}")]
    Invalid(&'static str),
}
