//! Contract for `examples/pipelined_macro.rs`.

use std::fmt;
use std::future::IntoFuture;

use typestate_pipeline::{InFlight, Pipelined, Resolved, pipelined};

#[derive(Debug, Default)]
pub struct Hub;

#[derive(Debug)]
pub struct AppError;
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("err")
    }
}
impl std::error::Error for AppError {}

pipelined!(pub Author, ctx = Hub, error = AppError);

#[derive(Debug)]
pub struct Phase1;

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    // `Pipelined` impl exposes the documented associated types.
    fn check_pipelined<'a>()
    where
        Author<'a, Phase1, Resolved>: Pipelined<
                'a,
                Ctx = Hub,
                Error = AppError,
                Tag = (),
                State = Phase1,
                Mode = Resolved,
            >,
        Author<'a, Phase1, InFlight>: Pipelined<'a, State = Phase1, Mode = InFlight>,
    {
    }
    let _: fn() = check_pipelined::<'static>;

    // `IntoFuture` exists on the InFlight carrier with the documented Output.
    fn check_intofuture<'a>()
    where
        Author<'a, Phase1, InFlight>:
            IntoFuture<Output = Result<Author<'a, Phase1, Resolved>, AppError>>,
    {
    }
    let _: fn() = check_intofuture::<'static>;
}

#[test]
fn surface_compiles() {
    surface_check();
}
