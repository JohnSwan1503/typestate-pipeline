//! Compile-fail tests locking down the wording of macro diagnostics.
//!
//! Each `tests/ui/*.rs` file is a minimal program that should fail to compile
//! with a specific macro error. The matching `tests/ui/*.stderr` file pins
//! down the exact rustc output. These guard against silent drift in error
//! wording when the macros change — when the message intentionally improves,
//! regenerate the stderr files with:
//!
//!     TRYBUILD=overwrite cargo test --test ui_macros
//!
//! and review the diff before committing.
//!
//! These tests run rustc once per file and are slower than the rest of the
//! suite (~seconds), but they're cheap insurance against accidentally
//! breaking user-facing diagnostics.
//!
//! ## Feature scoping
//!
//! Some UI tests (notably `ready_trait_rejects_unset_required`) inspect
//! rustc's "method not found" diagnostic, which lists *all* in-scope
//! traits with a matching method name as candidates. When the
//! `dataset-authoring-example` feature is enabled, the `dataset_authoring`
//! module compiles and emits its own `*Ready` traits — those then appear
//! in the candidates list and the recorded stderr diverges from the
//! feature-off recording. Rather than maintain two parallel stderr files,
//! we pin these tests to the feature-off configuration; the
//! `dataset-authoring-example` feature has its own integration-level
//! coverage in `tests/integration/dataset_authoring.rs`.
//!
//! The `no_unsafe` feature is also excluded for an inverse reason:
//! `tests/ui/factory_no_unsafe_without_feature.rs` exists *because*
//! `#[factory(no_unsafe)]` should be rejected when the feature is off.
//! With the feature on the attribute compiles cleanly, so trybuild would
//! flag a "test expected to fail but compiled" mismatch. Run with the
//! feature on if you only want the safe-mode coverage in
//! `tests/safety/factory_no_unsafe.rs`.

#[cfg(not(any(
    feature = "dataset-authoring-example",
    feature = "no_unsafe",
)))]
#[test]
fn ui_compile_failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
