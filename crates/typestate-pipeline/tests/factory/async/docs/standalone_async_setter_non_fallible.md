## Async Setter

**Invariant.** A standalone `#[field(setter = ..., async_fn)]`
returns a future. Awaiting the future yields the next bag with the
field's flag advanced from `No` to `Yes` and the transformed value
stored. Pairing with `fallible` makes the awaited result a
`Result<NextBag, Error>` instead.

**Failure mode this guards.** A buggy codegen could:

- Treat the setter as sync (no future at all) — call site would
  fail to compile because the chain expects `.await`.
- Defer the bag advancement past `.await` — observed flag would
  still be `No` after awaiting, breaking subsequent setters.
- Skip the transformer — `name` would be the raw input, not the
  trimmed value.

**Setup.** `UserProfile` with `name` (async non-fallible, trims
whitespace) and `email` (async fallible, validates non-empty,
lowercases). Chain: `name("  Alice  ").await -> email("Alice@Example.COM").await.expect(...) -> finalize()`.

**Assertion.** `user.name == "Alice"` (trimmed) and
`user.email == "alice@example.com"` (lowercased). The fallible
setter's `Ok(...)` was unwrapped via `expect`.

### standalone_async_setter_non_fallible
