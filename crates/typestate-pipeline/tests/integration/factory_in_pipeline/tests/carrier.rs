use typestate_pipeline::{Pipeline, Resolved, pipelined, transitions};

use super::domain::{Server, User, UserFactory};
use super::error::SubmitError;
use super::phases::{Confirmed, Drafting, Submitted};

pipelined!(pub Author, ctx = Server, error = SubmitError);

impl<'a, S: 'a> Author<'a, S, Resolved> {
    pub fn state(&self) -> &S {
        self.0.state()
    }
}

#[transitions(error = SubmitError)]
impl<'a> Author<'a, Drafting> {
    /// Finalize the bag, validate the resulting `User`, and advance.
    #[transition(into = Submitted)]
    pub fn submit(state: Drafting) -> Result<Submitted, SubmitError> {
        let user = state.0.finalize();
        if user.name.is_empty() {
            return Err(SubmitError::Empty("name"));
        }
        if user.email.is_empty() {
            return Err(SubmitError::Empty("email"));
        }
        Ok(Submitted(user))
    }
}

#[transitions(error = SubmitError)]
impl<'a> Author<'a, Submitted> {
    #[transition(into = Confirmed)]
    pub async fn confirm(state: Submitted, ctx: &Server) -> Result<Confirmed, SubmitError> {
        let id = ctx.confirm(&state.0);
        Ok(Confirmed {
            user: state.0,
            confirmation_id: id,
        })
    }
}

// ---------------------------------------------------------------------------
// Test helper — build a fully-set bag from a `User` and walk into the
// carrier in `Drafting` mode. Real code would build the bag via
// setters interactively; here we round-trip for compactness.
// ---------------------------------------------------------------------------

pub fn drafting<'a>(server: &'a Server, user: User) -> Author<'a, Drafting> {
    let bag = UserFactory::new()
        .name(user.name)
        .email(user.email)
        .with_age(user.age);
    Author(Pipeline::resolved(server, Drafting(bag)))
}
