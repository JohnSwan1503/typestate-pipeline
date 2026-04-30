#[path = "error.rs"]
pub mod error;

#[path = "phases.rs"]
pub mod phases;

#[path = "carrier.rs"]
pub mod carrier;

pub use carrier::{Author, Client};
pub use error::TestError;
pub use phases::{Deployed, JobConfigured, Registered, Versioned};
