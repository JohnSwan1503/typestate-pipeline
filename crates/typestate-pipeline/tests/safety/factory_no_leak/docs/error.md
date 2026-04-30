### Error type

A minimal `std::error::Error` whose only purpose is to act as a sentinel
that a fallible setter has rejected its input. The tests don't care about
the error's identity, only that it propagates and that the failing path
released every previously-set field on the bag.

`Reject` is intentionally trivial — no payload, `Display` writes a fixed
string, no `Clone` / `PartialEq`. Anything richer would distract from
the invariant under test (drop-bookkeeping correctness).
