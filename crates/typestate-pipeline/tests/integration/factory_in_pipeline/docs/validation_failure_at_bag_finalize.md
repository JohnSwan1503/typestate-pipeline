## Bag-Finalize Short-Circuit

**Invariant.** When the bag-finalize step inside a sync-fallible
transition produces `Err`, the resolved arm returns the error directly
at the call site — no future is constructed, no later transition runs.

**Failure mode this guards.** A naive codegen could lift the entire
sync-fallible body into a future eagerly (always producing
`Author<Next, InFlight>`) so the chain folds uniformly. That would
defer the error to the terminal `.await?`, hiding sync errors behind
async machinery. The macro instead emits two separate Resolved-arm
shapes:

- `Resolved -> Result<Resolved, E>` for sync fallible (you see the
  error at the call site).
- `Resolved -> InFlight` for async deferred.

This test pins the Resolved-arm-of-sync-fallible shape: a `match` on
the result yields the validation error immediately, before any
`confirm` future could exist.

**Setup.** A `User` with `name = ""` (empty), which is the first field
the `submit` body validates against.

**Assertion.** `pipeline.submit()` returns `Err(SubmitError::Empty("name"))`
synchronously. The match in the test never reaches the `Ok` branch.

### validation_failure_at_bag_finalize
