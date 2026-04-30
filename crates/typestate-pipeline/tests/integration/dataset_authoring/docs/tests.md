# Dataset-authoring end-to-end tests

End-to-end integration tests for the rebuilt dataset-authoring pipeline
shipped under the `dataset-authoring-example` Cargo feature. The five
tests mirror the four upstream `amp-client-admin` example flows, but
as regression-locked unit tests so the chain shapes (and their typed
errors) can't quietly drift.

Each test asserts a concrete observable property on top of the
in-memory mock `Client`:

- The new-evm-rpc happy path produces `job_id == 1` and the expected
  `Reference`.
- The new-derived happy path also produces `job_id == 1`, with the
  multi-async-step chain folding into a single terminal `.await?`.
- An `edit_existing_derived` flow with `bump_patch()` arithmetically
  increments the existing version (`0.1.0 -> 0.1.1`).
- `bump_patch()` against a dataset with no prior tagged version
  surfaces `AuthoringError::NoPriorVersion`.
- `edit_existing_derived` against a dataset that's actually `evm_rpc`
  surfaces `AuthoringError::KindMismatch { expected: "derived", .. }`.

The whole suite is gated behind `#[cfg(feature = "dataset-authoring-example")]`
so the optional `serde` / `serde_json` / `thiserror` deps the demo
relies on don't pollute normal `cargo test` runs.
