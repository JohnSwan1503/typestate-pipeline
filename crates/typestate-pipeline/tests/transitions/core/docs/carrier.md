### Carrier + transitions

`Client` is the mock context — an `AtomicU64` `next_job_id` that
hands out monotonically-increasing job ids so tests can assert
deterministic values (`job_id == 1`).

`Author` is the carrier (declared via `pipelined!`). It carries
two convenience accessors:

- `from_registered(client, name, manifest_hash)` — opens the
  carrier in `Registered` mode with a freshly-built state.
- `state(&self) -> &S` — borrowed access to the state on a
  Resolved carrier.

Three `#[transitions]` impls cover all four body shapes:

- `Author<Registered>` declares `tag_version` (async deferred)
  and `confirm_and_tag` (async breakpoint, `breakpoint`).
- `Author<Versioned>` declares `with_parallelism` (sync
  infallible).
- `Author<JobConfigured>` declares `validate_and_finalize` (sync
  fallible) and `deploy` (async deferred).

Each test body chains a different combination of these
transitions to exercise its particular property.
