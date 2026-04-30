### Shared infrastructure

- `Hub` — context type with an `AtomicU64` counter so `allocate()`
  hands back deterministic ids (1, 2, …).
- `AppError` — uninhabited enum (the test suite never produces an
  error). Has `Display` and `Error` impls because the carrier's
  `error =` slot requires a real error type.
- `Author` — carrier declared with `pipelined!(pub Author, ctx = Hub,
  error = AppError)`. The `inspect` combinator under test is part of
  `pipelined!`'s emitted code.
- `Drafted` / `Tagged` / `Deployed` — phase-state types.
- Two `#[transitions]` impls (`tag` on `Drafted`, `deploy` on
  `Tagged`) — both async-deferred, so calling either one opens an
  InFlight chain. That's what lets us observe the InFlight-arm
  `inspect`.
- `state()` / `into_state()` accessors and `drafted(...)` constructor
  on the carrier — needed because the inner `Pipeline` field is
  private.
