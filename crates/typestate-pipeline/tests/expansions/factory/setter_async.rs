//! Contract for `examples/factory_setter_async.rs`.

use std::future::Future;

use typestate_pipeline::{No, TypestateFactory, Yes};

#[derive(Debug)]
#[allow(dead_code)]
pub struct BadInput;

#[derive(TypestateFactory)]
#[factory(error = BadInput)]
#[allow(dead_code)]
struct User {
    #[field(required, setter = normalize_async, async_fn)]
    name: String,
    #[field(required, setter = validate_email_async, async_fn, fallible)]
    email: String,
}

async fn normalize_async(value: String) -> String {
    value
}
async fn validate_email_async(value: String) -> Result<String, BadInput> {
    Ok(value)
}

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    let _: fn() -> UserFactory<No, No> = UserFactory::new;

    // async setter — returns `impl Future<Output = NextBag>`. Capture the
    // shape via a generic helper bound on `Future` to keep the contract
    // independent of the concrete future type.
    fn _check_async_infallible<Fut>(_: fn(UserFactory<No, No>, String) -> Fut)
    where
        Fut: Future<Output = UserFactory<Yes, No>>,
    {
    }
    _check_async_infallible(<UserFactory<No, No>>::name);

    // async fallible setter — `impl Future<Output = Result<NextBag, Error>>`.
    fn _check_async_fallible<Fut>(_: fn(UserFactory<No, No>, String) -> Fut)
    where
        Fut: Future<Output = Result<UserFactory<No, Yes>, BadInput>>,
    {
    }
    _check_async_fallible(<UserFactory<No, No>>::email);

    let _: fn(UserFactory<Yes, Yes>) -> User = <UserFactory<Yes, Yes>>::finalize;
}

#[test]
fn surface_compiles() {
    surface_check();
}
