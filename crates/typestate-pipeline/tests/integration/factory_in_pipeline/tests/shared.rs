#[path = "error.rs"]
pub mod error;

#[path = "domain.rs"]
pub mod domain;

#[path = "phases.rs"]
pub mod phases;

#[path = "carrier.rs"]
pub mod carrier;

pub use carrier::{Author, drafting};
pub use domain::{Server, User, UserFactory};
pub use error::SubmitError;
pub use phases::{Confirmed, Drafting, Submitted};
