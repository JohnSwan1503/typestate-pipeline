### Shared infrastructure

- `Hub` — minimal context type, just an opaque marker.
- `AppError::Bad(&'static str)` — typed error read off the carrier's
  `Pipelined::Error` projection.
- `Author` — carrier declared with `pipelined!(pub Author, ctx = Hub,
  error = AppError)`. The macro emits the `Pipelined<'a>` impl that
  the suite's tests rely on.
- `into_state()` accessor on the Resolved carrier — needed because
  the inner `Pipeline` field is private to `pipelined!`'s scope; the
  per-test files reach finalized values through this helper.
- `Drafted` / `Versioned` / `Published` — phase-state types.
- Two `#[transitions]` impls (both with `error =` *omitted*) cover
  one async-deferred (`tag`) and one sync-fallible (`publish`) body.
- `Profile` — a `TypestateFactory` derive with
  `#[factory(pipeline(carrier = Author))]` and no `error =`. Pinned
  as the no-fallible-setter case so the macro doesn't require an
  error type at the bag site either.
