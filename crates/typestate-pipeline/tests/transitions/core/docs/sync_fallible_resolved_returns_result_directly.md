## Sync Fallible Resolved Arm

**Invariant.** A sync-fallible transition (`fn -> Result<T, E>`) on
a Resolved carrier returns `Result<Author<Next, Resolved>, E>` —
not a future. The error is at the call site; no `.await` is
involved.

**Failure mode this guards.** A naive codegen could lift every
fallible body into a future "for chain-folding consistency," forcing
the user to `.await?` even when the underlying body is sync. That's
unergonomic and obscures the sync nature of the transition. The
test pins that the Resolved arm is genuinely sync.

**Setup.** Reach `Versioned` via the breakpoint
(`confirm_and_tag().await?`), then call sync-infallible
`with_parallelism(2)` (still Resolved), then sync-fallible
`validate_and_finalize()` and `.unwrap()` the result directly.

**Assertion.** The unwrapped carrier's state has `verified == true`
(set inside the body) and `parallelism == 2`. No `.await` was used
on `validate_and_finalize`.

### sync_fallible_resolved_returns_result_directly
