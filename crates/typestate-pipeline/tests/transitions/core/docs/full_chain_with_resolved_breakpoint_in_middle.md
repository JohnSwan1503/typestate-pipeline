## Full Chain Folding

**Invariant.** A chain that mixes async-deferred, sync-infallible,
sync-fallible, and async-deferred transitions in sequence folds into
a single terminal `.await?`. No mid-chain awaits, no mid-chain
`Result` unwraps.

**Failure mode this guards.** This is the canonical happy-path test
for `#[transitions]`'s codegen. It pins that:

- **Async deferred** lifts `Resolved -> InFlight` and stays InFlight
  for the rest of the chain.
- **Sync infallible** in the middle of an InFlight chain folds in via
  `Pipeline::map_inner_sync`.
- **Sync fallible** in the middle of an InFlight chain folds its
  `Result` into the pending future via
  `Pipeline::map_inner_sync_fallible` — the error surfaces at the
  terminal `.await?`, not at the call site.
- **Async deferred** at the end stays InFlight and finally resolves
  at the user's terminal `.await`.

If any of those shapes generated a wrong arm (e.g. forced a
mid-chain `.await` for the sync-fallible step), this test would
either fail to compile or require a chain rewrite.

**Setup.** Start with a fresh `Author<Registered, Resolved>` from
`Client::default()`. Chain `tag_version(7) -> with_parallelism(8) ->
validate_and_finalize() -> deploy()`, terminating with a single
`.await`.

**Assertion.** The deployed state's `name == "ds-a"`, `version == 7`,
and `job_id == 1` (first id from the mock client).

### full_chain_with_resolved_breakpoint_in_middle
