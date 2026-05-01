//! Compile-fail tests locking down the wording of macro diagnostics.
//!
//! Each `tests/ui/<channel>/<name>.rs` file is a minimal program that
//! should fail to compile with a specific macro error. The matching
//! `<name>.stderr` next to it pins the exact rustc output for that
//! channel. These guard against silent drift in error wording when the
//! macros change — when the message intentionally improves, regenerate
//! the relevant fixture set with:
//!
//!     TRYBUILD=overwrite cargo test --test ui_macros
//!
//! and review the diff before committing.
//!
//! ## Channel split
//!
//! rustc's diagnostic format drifts between channels — nightly often
//! adds suggestion lines, secondary spans, or error-code links that
//! stable doesn't emit. Rather than fight this with one shared fixture
//! set, the `tests/ui/` tree is split:
//!
//!   - `tests/ui/stable/` — fixtures pinned to the rendering produced
//!     by the current stable toolchain. Exercised by [`ui_compile_failures_stable`].
//!   - `tests/ui/nightly/` — fixtures pinned to nightly's rendering.
//!     Exercised by [`ui_compile_failures_nightly`]. The `.rs` source
//!     files are byte-identical to `stable/`; only the `.stderr`
//!     fixtures differ. Sources are duplicated rather than symlinked
//!     to keep the layout cross-platform.
//!
//! Each `#[test]` function is gated by `#[rustversion::…]` so the
//! channel that doesn't match its fixture set silently skips. When
//! adding a new compile-fail case, add the `.rs` file to *both*
//! `stable/` and `nightly/`, then bless each on the matching toolchain.
//!
//! These tests run rustc once per file and are slower than the rest of
//! the suite (~seconds), but they're cheap insurance against
//! accidentally breaking user-facing diagnostics.
//!
//! ## Feature scoping
//!
//! Some UI tests (notably `ready_trait_rejects_unset_required`) inspect
//! rustc's "method not found" diagnostic, which lists *all* in-scope
//! traits with a matching method name as candidates. When the
//! `dataset-authoring-example` feature is enabled, the `dataset_authoring`
//! module compiles and emits its own `*Ready` traits — those then appear
//! in the candidates list and the recorded stderr diverges from the
//! feature-off recording. Rather than maintain two parallel stderr files
//! per channel, we pin these tests to the feature-off configuration; the
//! `dataset-authoring-example` feature has its own integration-level
//! coverage in `tests/integration/dataset_authoring.rs`.
//!
//! The `no_unsafe` feature is also excluded for an inverse reason:
//! `factory_no_unsafe_without_feature.rs` exists *because*
//! `#[factory(no_unsafe)]` should be rejected when the feature is off.
//! With the feature on the attribute compiles cleanly, so trybuild would
//! flag a "test expected to fail but compiled" mismatch. Run with the
//! feature on if you only want the safe-mode coverage in
//! `tests/safety/factory_no_unsafe.rs`.

#[rustversion::stable]
#[cfg(not(any(feature = "dataset-authoring-example", feature = "no_unsafe")))]
#[test]
fn ui_compile_failures_stable() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/stable/*.rs");
}

#[rustversion::nightly]
#[cfg(not(any(feature = "dataset-authoring-example", feature = "no_unsafe")))]
#[test]
fn ui_compile_failures_nightly() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/nightly/*.rs");
}
