//! Contract for `examples/impl_pipelined_macro.rs`.

use std::fmt;
use std::future::IntoFuture;

use typestate_pipeline::{
    InFlight, Mode, Pipeline, Pipelined, Resolved, impl_pipelined,
};

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

pub struct Author<'a, S, M = Resolved>(pub Pipeline<'a, Hub, (), S, AppError, M>)
where
    M: Mode<'a, S, AppError>;

impl_pipelined!(Author, ctx = Hub, error = AppError);

#[derive(Debug)]
pub struct Phase1;

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
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
    {
    }
    let _: fn() = check_pipelined::<'static>;

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
