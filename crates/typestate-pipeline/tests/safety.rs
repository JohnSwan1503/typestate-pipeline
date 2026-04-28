//! Safety-focused integration tests: drop/leak invariants and the
//! `no_unsafe` codegen path. Compiled into a single test binary so the
//! per-module `ALIVE` counters stay independent and the linker step runs
//! once for the whole domain.

#[path = "safety/factory_hygiene.rs"]
mod factory_hygiene;
#[path = "safety/factory_no_leak.rs"]
mod factory_no_leak;
#[path = "safety/factory_no_unsafe.rs"]
mod factory_no_unsafe;
#[path = "safety/factory_panic_safety.rs"]
mod factory_panic_safety;
#[path = "safety/factory_phantom_shape.rs"]
mod factory_phantom_shape;
