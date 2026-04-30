use super::domain::{User, UserFactory};

// Phase-state types. `Drafting` carries a fully-set bag; `Submitted`
// and `Confirmed` carry the finalized values produced by the
// transitions.

pub struct Drafting(
    pub UserFactory<typestate_pipeline::Yes, typestate_pipeline::Yes, typestate_pipeline::Yes>,
);

pub struct Submitted(pub User);

pub struct Confirmed {
    pub user: User,
    pub confirmation_id: u64,
}
