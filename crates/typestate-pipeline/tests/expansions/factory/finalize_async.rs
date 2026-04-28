//! Contract for `examples/factory_finalize_async.rs`.

use std::future::Future;

use typestate_pipeline::{TypestateFactory, Yes};

#[derive(Debug)]
#[allow(dead_code)]
pub struct BadInput;

#[derive(Debug)]
#[allow(dead_code)]
struct ConfirmedUser {
    name: String,
    confirmation_token: String,
}

async fn confirm_user(raw: User) -> Result<ConfirmedUser, BadInput> {
    Ok(ConfirmedUser {
        confirmation_token: raw.name.clone(),
        name: raw.name,
    })
}

#[derive(TypestateFactory)]
#[factory(
    error = BadInput,
    finalize_async(via = confirm_user, into = ConfirmedUser, error = BadInput),
)]
#[allow(dead_code)]
struct User {
    #[field(required)]
    name: String,
}

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    // Inherent `finalize` still exists.
    let _: fn(UserFactory<Yes>) -> User = <UserFactory<Yes>>::finalize;

    // `finalize_async` is async; its return is the configured
    // `Result<into, error>` because `error = …` was supplied.
    fn _check_finalize_async<Fut>(_: fn(UserFactory<Yes>) -> Fut)
    where
        Fut: Future<Output = Result<ConfirmedUser, BadInput>>,
    {
    }
    _check_finalize_async(<UserFactory<Yes>>::finalize_async);
}

#[test]
fn surface_compiles() {
    surface_check();
}
