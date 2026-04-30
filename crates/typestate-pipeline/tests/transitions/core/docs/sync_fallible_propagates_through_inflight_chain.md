## Sync Fallible InFlight Folding

**Invariant.** A sync-fallible transition mid-chain (when the
previous step was async-deferred so we're already InFlight) folds
its `Result` into the pending future. The error doesn't surface at
the call site — it surfaces at the chain's terminal `.await?`.

**Failure mode this guards.** Symmetric to the Resolved-arm test:
the InFlight arm of a sync-fallible body has to *fold* the `Result`
into the chained pending future, not lift it to the call site.
A wrong codegen here would force the user to write
`.unwrap_or_else(...).await` mid-chain — defeating chain-folding.

**Setup.** Start at `Registered`, advance via async-deferred
`tag_version(1)` (now InFlight), call sync-infallible
`with_parallelism(0)` (folds in, still InFlight), then
sync-fallible `validate_and_finalize()` (returns `Err` because
`parallelism == 0`), then async-deferred `deploy()`. Terminal
`.await`.

**Assertion.** The terminal `.await` returns `Err(Invalid(...))`
with the validate's exact message — proving the error folded
through the InFlight chain rather than being lost or surfacing too
early.

### sync_fallible_propagates_through_inflight_chain
