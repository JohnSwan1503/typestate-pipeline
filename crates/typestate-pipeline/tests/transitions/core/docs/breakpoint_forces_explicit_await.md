## Async Breakpoint

**Invariant.** An async transition decorated with
`#[transition(breakpoint)]` (an *async breakpoint*) returns
`async fn -> Result<Resolved, E>` from both arms. The chain breaks
at this point — the next link sees a `Resolved` carrier whose state
is fully observable, not a pending future.

**Failure mode this guards.** Two separate properties:

1. **Resolved-arm shape.** The Resolved arm of an async breakpoint
   must NOT lift to InFlight (which is what async-deferred does).
   If the codegen forgot to special-case `breakpoint` on the
   Resolved input, calling `confirm_and_tag(...)` on a Resolved
   carrier would return InFlight — defeating the breakpoint.
2. **InFlight-arm shape.** The InFlight arm must also resolve to
   `Resolved` (not stay InFlight). A user awaiting the InFlight
   result expects a Resolved carrier ready for synchronous follow-up
   work.

This test enters a Resolved breakpoint, awaits it, and then chains
a sync-fallible call (`validate_and_finalize`) — proving the
breakpoint really did hand back a Resolved carrier (sync-fallible's
Resolved arm hands back `Result` directly, no future indirection).

**Setup.** Fresh `Author<Registered, Resolved>`.
`confirm_and_tag().await?` advances to `Versioned`. Then
`with_parallelism(4).validate_and_finalize().expect(...)` advances
synchronously to `JobConfigured`. Finally `deploy().await?`.

**Assertion.** Deployed state has `version == 1` (set inside
`confirm_and_tag`) and `job_id == 1`.

### breakpoint_forces_explicit_await
