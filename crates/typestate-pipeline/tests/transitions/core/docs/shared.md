### Shared infrastructure

- `Client` — mock external system that hands out monotonically
  increasing `job_id`s. Tests assert specific values
  (e.g. `job_id == 1`) so the counter has to be deterministic.
- `TestError::Invalid(&'static str)` — typed validation error with a
  static message slot.
- `Registered` / `Versioned` / `JobConfigured` / `Deployed` — the
  four phase-state types the chain walks through.
- `Author` — the carrier newtype, declared via `pipelined!`, with a
  `from_registered(...)` constructor and a `state()` accessor.
- Three `#[transitions]` impls cover all four body shapes:
  - `Registered`: async deferred (`tag_version`) + async breakpoint
    (`confirm_and_tag`).
  - `Versioned`: sync infallible (`with_parallelism`).
  - `JobConfigured`: sync fallible (`validate_and_finalize`) + async
    deferred (`deploy`).

The whole test suite shares this carrier; each test writes only the
particular chain shape it's exercising.
