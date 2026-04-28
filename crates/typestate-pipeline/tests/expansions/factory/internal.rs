//! Contract for `examples/factory_internal.rs`.

use typestate_pipeline::{No, TypestateFactory, Yes};

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct Job {
    #[field(required, internal)]
    namespace: String,
    #[field(required)]
    parallelism: u16,
    #[field(default = false)]
    verify: bool,
}

#[allow(dead_code, unused_variables, clippy::type_complexity)]
fn surface_check() {
    // `new` takes the internal field positionally; the flag generics list
    // contains exactly the two non-internal fields (parallelism, verify).
    let _: fn(String) -> JobFactory<No, No> = JobFactory::new;

    // Setters are emitted only for non-internal fields.
    let _: fn(JobFactory<No, No>, u16) -> JobFactory<Yes, No> =
        <JobFactory<No, No>>::parallelism;
    let _: fn(JobFactory<No, No>, bool) -> JobFactory<No, Yes> =
        <JobFactory<No, No>>::with_verify;

    // Internal getter is unconditional — callable on every shape.
    let _: for<'a> fn(&'a JobFactory<No, No>) -> &'a String =
        <JobFactory<No, No>>::namespace;
    let _: for<'a> fn(&'a JobFactory<Yes, Yes>) -> &'a String =
        <JobFactory<Yes, Yes>>::namespace;

    // Finalize requires the two flag generics to be Yes / Satisfiable.
    let _: fn(JobFactory<Yes, Yes>) -> Job = <JobFactory<Yes, Yes>>::finalize;
    let _: fn(JobFactory<Yes, No>) -> Job = <JobFactory<Yes, No>>::finalize;
}

#[test]
fn surface_compiles() {
    surface_check();
}
