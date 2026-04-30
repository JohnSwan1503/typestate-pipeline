### Carrier + Pipeline-integrated bag

`Hub` is the context type for the carrier. `Author` is declared
via `pipelined!`. `Order` is the Pipeline-integrated bag — it
declares `pipeline(carrier = Author)` so the derive emits the
bag's setters directly on the carrier, in both `Resolved` and
`InFlight` modes.

`Order` lives in this same module (rather than next to the other
bags) because the derive's pipeline-arm codegen has to reach into
`Author`'s tuple-struct internals — keeping the two definitions
colocated is what makes that visibility work without further
plumbing.

The `Booked` phase + the `book` `#[transitions]` impl on the
fully-set bag exercises the "call `finalize()` mid-chain" pattern.
`empty_order` is the test entry point that wraps the
(otherwise-private) `Author` construction.
