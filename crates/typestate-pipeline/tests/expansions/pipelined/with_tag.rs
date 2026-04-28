//! Contract for `examples/pipelined_with_tag.rs`.

use core::fmt;

use typestate_pipeline::{Pipelined, Resolved, pipelined};

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

#[derive(Debug)]
pub enum RawKind {}

pipelined!(pub Author, ctx = Hub, error = AppError, tag = RawKind);

#[derive(Debug)]
pub struct Phase1;

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    // `Pipelined::Tag` is `RawKind`, not `()`.
    fn check_tag<'a>()
    where
        Author<'a, Phase1, Resolved>: Pipelined<'a, Tag = RawKind>,
    {
    }
    let _: fn() = check_tag::<'static>;
}

#[test]
fn surface_compiles() {
    surface_check();
}
