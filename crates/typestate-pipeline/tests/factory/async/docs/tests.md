# Async setter and async finalize coverage

Three families of async behavior, all locked in here:

1. **`#[field(setter = …, async_fn)]` standalone.** The setter
   returns a future; awaiting it advances the bag's flag. Combine
   with `fallible` for an async-fallible setter that returns
   `Result<NextBag, Error>` after `.await`.
2. **`#[factory(finalize_async(via = …, into = …, error = …?))]`.**
   Async finalize hook — coexists with the sync `finalize()`.
3. **Pipeline-integrated async setters.** With `#[factory(pipeline(carrier = …))]`,
   the async setter on the carrier opens an InFlight chain that
   subsequent setters fold into.

Plus a fourth scenario — the most flexible path for finalize: a
`#[transitions]` body that calls `finalize()` (or `finalize_async()`)
inline. Useful when the finalize work needs extra context (a Hub
reference here, but conceptually anything the carrier carries) or
produces different downstream phases conditionally.

The tests share three bags (UserProfile, User, Order) so each
async shape is exercised in isolation, and the carrier-based tests
share a single `Author` carrier.
