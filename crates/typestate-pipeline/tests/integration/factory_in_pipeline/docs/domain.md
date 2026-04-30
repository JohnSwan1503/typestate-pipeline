### Domain types

`Server` is a stub for an external system that can confirm a
`User` and return an id. It uses an `AtomicU64` so the test sees
a predictable confirmation id (`1` for the first call) without
locking.

`User` is the finalized domain object. It derives
`TypestateFactory`, generating `UserFactory<F1, F2, F3>` with one
flag per field. The factory has no `pipeline(carrier = …)` arm —
the bag is built outside the carrier and walked into `Drafting`
via the `drafting(...)` helper.
