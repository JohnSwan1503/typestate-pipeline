## InFlight `inspect` Folds

**Invariant.** The InFlight-arm `inspect` returns `Self` (still
InFlight) so subsequent transitions can append to the same pending
future. A chain like `.tag().inspect(...).deploy().await?` folds
into a single terminal `.await?`.

**Failure mode this guards.** A buggy codegen could lift the
InFlight `inspect` to Resolved (forcing a mid-chain `.await`),
defeating the chain-folding contract. The test pins the contract by
calling another async-deferred transition (`deploy`) after `inspect`
without an intermediate `.await`. If `inspect` returned Resolved,
calling `deploy` (which expects InFlight or Resolved input but
produces InFlight) would still typecheck — but the chain wouldn't
actually fold; it'd require two `.await`s. The test's single `.await`
proves folding works.

**Setup.** Fresh `Author<Drafted, Resolved>`. Chain `tag()`
(deferred async, → InFlight) → `inspect(|c| ...)` (deferred,
records `tag=N` into a `Mutex<Vec<String>>`) → `deploy()` (deferred
async, → InFlight) → terminal `.await`.

`Hub::allocate()` is called twice (once during `tag`, once during
`deploy`), so the deployed state has `tag = 1` and `job_id = 2`.

**Assertion.**

- Log == `["tag=1"]` — the inspect closure ran during the
  `tag → deploy` boundary, observing `tag = 1`.
- Deployed state has `name == "delta"`, `tag == 1`, `job_id == 2`.

### inflight_inspect_chains_through_subsequent_transitions
