### Shared infrastructure

- `BadInput::Empty` — single-variant error reused by every fallible
  path in the suite.
- `UserProfile` — bag with two async fields. `name` uses an
  async-non-fallible transformer (`normalize_name_async`); `email`
  uses an async-fallible transformer (`validate_email_async`).
- `User` + `ConfirmedUser` + `confirm_user` — bag with
  `finalize_async(via = confirm_user, into = ConfirmedUser, error = BadInput)`.
  The hook runs inside finalize and produces the post-confirm value.
- `Hub` — context type for the carrier-based tests.
- `Order` — bag with `pipeline(carrier = Author)` and async setters.
- `Author` — carrier declared via `pipelined!`. The pipeline-arm
  async setters open InFlight chains naturally.
- `Booked` + `book` — a `#[transitions]` body that calls
  `finalize()` mid-chain to produce the next phase. Demonstrates
  that finalize composes with carrier transitions.
- `state()` / `into_state()` accessors on the carrier and an
  `empty_order` constructor to wrap the (private-elsewhere) inner
  field.
