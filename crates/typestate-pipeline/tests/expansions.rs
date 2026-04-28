//! Compile-time contracts for the `// Generated (sketch)` blocks in
//! `examples/`. Each module below mirrors one example file and uses
//! function-pointer coercion to assert the macro emits the names and
//! signatures the sketch advertises.
//!
//! These tests run no code — the assertions are entirely in the type
//! system. A cargo build of this binary is the regression guard; if a
//! macro change renames a setter or flips a flag transition, the
//! corresponding `let _: fn(…) -> …` line fails to compile and the test
//! binary fails to build.
//!
//! The pattern: each module
//! 1. re-declares the same struct shape used in the matching example,
//! 2. defines a `surface_check` fn that fn-pointer-coerces every public
//!    item the example's sketch lists,
//! 3. calls `surface_check` from a `#[test]` so `cargo test` exercises it
//!    (the call itself is unreachable; compilation is the assertion).
#[path = "expansions/factory.rs"]
mod factory;

#[path = "expansions/transitions.rs"]
mod transitions;

#[path = "expansions/impl_pipelined.rs"]
mod impl_pipelined;

#[path = "expansions/pipelined.rs"]
mod pipelined;

#[path = "expansions/combo.rs"]
mod combo;
