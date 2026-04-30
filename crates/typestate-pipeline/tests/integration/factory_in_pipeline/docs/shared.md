### Shared infrastructure

Domain types, phase-state types, the carrier, and the transitions impls
that both tests draw from.

- `Server` is a stub for an external system that can confirm a `User`
  and hand back an id. It uses an `AtomicU64` so the test sees a
  predictable confirmation id (1 for the first call) without locking.
- `SubmitError` is the carrier's error type. Only one variant matters
  for this suite — `Empty(field_name)` for the validation failure path.
- `User` is the finalized domain object. It derives `TypestateFactory`,
  generating `UserFactory<F1, F2, F3>` with one flag per field.
- `Drafting` / `Submitted` / `Confirmed` are the three phase-state
  marker types. `Drafting` carries the fully-set bag; the other two
  carry the finalized values.
- `pipelined!(pub Author, ...)` declares the carrier newtype, its
  `Pipelined<'a>` impl, and `IntoFuture` forwarding for `InFlight`.
- The two `#[transitions]` impls fold `Drafting -> Submitted` (sync
  fallible) and `Submitted -> Confirmed` (async deferred), exercising
  the two body shapes most relevant to bag/pipeline composition.
- `drafting(...)` is the test entry point that builds a fully-set bag
  from a plain `User` and walks into the carrier.
