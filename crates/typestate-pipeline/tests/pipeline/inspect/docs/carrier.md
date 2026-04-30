### Carrier + transitions

`Hub` is the context. Its `next_id: AtomicU64` and
`allocate(&self)` give the chain monotonic ids the assertions
pin to (the tag tests use both `tag == 1` and `job_id == 2`).

`Author` is the carrier (declared via `pipelined!`). The
`inspect` combinator under test is part of `pipelined!`'s
emitted code.

Two `#[transitions]` impls — `tag` on `Drafted` and `deploy` on
`Tagged`, both async-deferred. That's what lets every test open
an InFlight chain just by calling `tag()`, which is what the
deferred `inspect` arm depends on.

`drafted(hub, name)` wraps the (otherwise-private) Author
construction so per-test files don't have to reach into the
carrier's tuple-field internals.
