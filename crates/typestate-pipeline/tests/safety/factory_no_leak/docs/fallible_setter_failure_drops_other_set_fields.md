## Fallible Setter Failure

**Invariant.** When a `fallible` setter rejects its input via `?`, the
caller's bag must drop every field that was already set on it. The bag's
state has not advanced (the type-level flag for the failing field is
still `No`), so the bag's normal `Drop` impl is responsible for the
already-set fields.

**Failure mode this guards.** The naive codegen wrapped `self` in
`ManuallyDrop` *before* invoking the (possibly-fallible, possibly-async)
transformer. With that ordering, a `?`-short-circuit returned from
*inside* the `ManuallyDrop` scope; `self` was suppressed, and the
already-set fields' `MaybeUninit` slots were never dropped. The
regression: setting `other` and then calling `main(...)` with a value
the transformer rejects would leak `other` until the test's binary
exited.

**Setup.**

- `SyncFallibleBag` — two required `Counted` fields. `main`'s setter is
  `sync_reject`, which always returns `Err(Reject)` after dropping its
  input.
- `other` is set first; the test snapshots `alive() == baseline + 1`.
- `main(Counted::new("m"))` is called; `m` is consumed and dropped by the
  transformer.

**Assertion.** After the failing setter returns `Err`, `alive()` is
back to `baseline` — the bag's `Drop` ran on `other`. With the buggy
codegen this would be `baseline + 1`.

### fallible_setter_failure_drops_other_set_fields
