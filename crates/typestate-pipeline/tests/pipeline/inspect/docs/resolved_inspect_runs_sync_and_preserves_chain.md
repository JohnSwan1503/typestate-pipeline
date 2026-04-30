## Resolved `inspect` Sync

**Invariant.** `inspect(|c| ...)` on a Resolved carrier runs the
closure synchronously, observing the carrier's current state. The
carrier is returned unchanged — same type, same state.

**Failure mode this guards.** Two failure shapes:

1. **Inspect doesn't run.** A buggy codegen could emit `inspect` as a
   no-op on the Resolved arm (mistakenly treating it as
   InFlight-deferred). The closure would never execute and the
   `RefCell` observation would stay empty.
2. **Inspect runs at the wrong time.** Same closure could be deferred
   even on Resolved, so the observation would be stored *after* the
   immediately-following assertion runs.

**Setup.** A fresh `Author<Drafted, Resolved>` from
`drafted(&hub, "alpha")`. A `RefCell<Option<String>>` to record the
closure's observation. Call `.inspect(...)` and immediately assert.

**Assertion.** `observed.borrow().as_deref() == Some("alpha")` —
synchronously, before the test ends.

### resolved_inspect_runs_sync_and_preserves_chain
