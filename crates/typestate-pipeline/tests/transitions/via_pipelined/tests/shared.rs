#[path = "error.rs"]
pub mod error;

#[path = "phases.rs"]
pub mod phases;

#[path = "carrier.rs"]
pub mod carrier;

pub use carrier::{Author, Hub, Profile, ProfileFactory, drafted, drafted_inflight, empty_profile};
pub use error::AppError;
pub use phases::{Drafted, Published, Versioned};
