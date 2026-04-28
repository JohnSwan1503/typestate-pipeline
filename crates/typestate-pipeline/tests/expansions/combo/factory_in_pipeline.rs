//! Contract for `examples/combo_factory_in_pipeline.rs`.
//!
//! Both `TypestateFactory` and `#[transitions]` are exercised in the
//! same module; the combo's contract is that BOTH macros cooperate
//! without diagnostics or path-resolution failures. Per-feature
//! surfaces themselves are already locked in by the single-feature
//! contracts; here we only need the cohabitation to compile.

use core::fmt;

use typestate_pipeline::{No, Resolved, TypestateFactory, Yes, pipelined, transitions};

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

#[derive(TypestateFactory)]
#[allow(dead_code)]
pub struct Settings {
    #[field(required)]
    parallelism: u16,
    #[field(default = false)]
    verify: bool,
}

pipelined!(pub Author, ctx = Hub, error = AppError);

#[derive(Debug)]
pub struct Configured;
#[derive(Debug)]
pub struct Deployed;

#[transitions(error = AppError)]
impl<'a> Author<'a, Configured> {
    #[transition(into = Deployed)]
    pub fn deploy(state: Configured, ctx: &'a Hub) -> Deployed {
        let _ = (state, ctx);
        Deployed
    }
}

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    // Bag surface still works.
    let _: fn() -> SettingsFactory<No, No> = SettingsFactory::new;
    let _: fn(SettingsFactory<Yes, No>) -> Settings = <SettingsFactory<Yes, No>>::finalize;

    // Carrier transition still works.
    fn check_deploy<'a>(
        carrier: Author<'a, Configured, Resolved>,
    ) -> Author<'a, Deployed, Resolved> {
        carrier.deploy()
    }
    let _: for<'a> fn(Author<'a, Configured, Resolved>) -> Author<'a, Deployed, Resolved> =
        check_deploy;
}

#[test]
fn surface_compiles() {
    surface_check();
}
