### Shared infrastructure

- `Server` — stub external system that hands out monotonically-
  increasing `job_id`s. `AtomicU64` so the assertions can pin specific
  values without coordinating locks.
- `AppError` — carrier error type, only one variant matters here
  (`Empty(field_name)` for the fallible-setter rejection path).
- `DatasetData` is the bag. It deliberately exercises four different
  `#[field(...)]` shapes so the generated pipeline arm spans every
  method kind — bare setter, removable, defaulted-overridable, fallible
  with a transformer.
- `Author` is the carrier. `pipelined!` declares the newtype,
  `Pipelined<'a>` impl, `IntoFuture` for `InFlight`, and the
  `inspect` combinator. The `state()` shim is purely for convenience
  in the test bodies — equivalent to `self.0.state()` directly.
- The follow-on phase uses a `#[transitions]` impl whose self-type
  pins the bag's flag tuple to `(Yes, Yes, No, Yes)`. The strict bound
  is intentional: it demonstrates that a transition can operate on a
  specific bag shape, with the type system rejecting any call site
  whose bag doesn't match.
- `empty_bag(server)` is the test entry point: a fresh `DatasetDataFactory`
  wrapped in a `Resolved` carrier with the given server context.
