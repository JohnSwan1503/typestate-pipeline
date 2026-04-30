# Safe-mode (`#[factory(no_unsafe)]`) coverage

Mirrors every scenario from the unsafe-mode test suites — construction
and ordering, drop semantics, removable / overridable, conditional
finalize, custom transformers (sync infallible, sync fallible, async),
leak-safety on transformer failure, and internal fields — with every
derive opting into the safe codegen path so the generated bag uses the
`Storage<T>` associated-type trick (and therefore zero `MaybeUninit`,
zero `unsafe`, and zero manual `Drop` machinery).

The whole suite is gated behind `#[cfg(feature = "no_unsafe")]`. When
the feature is off, the `#[factory(no_unsafe)]` attribute is rejected
at expansion time so this file would fail to compile. The
corresponding negative trybuild case lives in
[`tests/ui/factory_no_unsafe_without_feature.rs`](../ui/factory_no_unsafe_without_feature/index.html).

The point of duplicating the unsafe-mode suite verbatim under the safe
codegen path is to prove that *every* user-visible property the
unsafe path delivers is also delivered by the safe path. Public method
names, signatures, and observable behavior are identical; only the
internals differ. A regression in the safe-mode codegen surfaces here
as a count mismatch (drop-bookkeeping tests) or a value mismatch
(round-trip tests), exactly mirroring how it would surface in the
unsafe-mode suite.
