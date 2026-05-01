//! Contract for `examples/factory_pipeline_arm.rs`.

use core::fmt;

use typestate_pipeline::{InFlight, No, Resolved, TypestateFactory, Yes, pipelined};

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

#[derive(TypestateFactory)]
#[factory(error = AppError, pipeline(carrier = Author))]
#[allow(dead_code)]
pub struct Order {
    #[field(required)]
    sku: String,
    #[field(default = 1, overridable)]
    quantity: u32,
}

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    // Resolved-mode setters on the carrier. The lifetime on the carrier
    // makes a HRTB fn-pointer awkward, so each check is wrapped in a
    // lifetime-parametric helper that names the expected signature, and
    // we hand the helper the actual method as a fn-item.
    fn check_resolved_required_setter<'a>(
        _: fn(
            Author<'a, OrderFactory<No, No>, Resolved>,
            String,
        ) -> Author<'a, OrderFactory<Yes, No>, Resolved>,
    ) {
    }
    check_resolved_required_setter(<Author<'_, OrderFactory<No, No>, Resolved>>::sku);

    fn check_resolved_optional_setter<'a>(
        _: fn(
            Author<'a, OrderFactory<No, No>, Resolved>,
            u32,
        ) -> Author<'a, OrderFactory<No, Yes>, Resolved>,
    ) {
    }
    check_resolved_optional_setter(<Author<'_, OrderFactory<No, No>, Resolved>>::with_quantity);

    fn check_resolved_default_helper<'a>(
        _: fn(
            Author<'a, OrderFactory<No, No>, Resolved>,
        ) -> Author<'a, OrderFactory<No, Yes>, Resolved>,
    ) {
    }
    check_resolved_default_helper(<Author<'_, OrderFactory<No, No>, Resolved>>::quantity_default);

    // Overridable arm — input bag's flag is Yes, output stays Yes.
    fn check_resolved_override<'a>(
        _: fn(
            Author<'a, OrderFactory<No, Yes>, Resolved>,
            u32,
        ) -> Author<'a, OrderFactory<No, Yes>, Resolved>,
    ) {
    }
    check_resolved_override(<Author<'_, OrderFactory<No, Yes>, Resolved>>::override_quantity);

    // InFlight-mode setter; same flag transitions, same name.
    fn check_inflight_setter<'a>(
        _: fn(
            Author<'a, OrderFactory<No, No>, InFlight>,
            String,
        ) -> Author<'a, OrderFactory<Yes, No>, InFlight>,
    ) {
    }
    check_inflight_setter(<Author<'_, OrderFactory<No, No>, InFlight>>::sku);
}

#[test]
fn surface_compiles() {
    surface_check();
}
