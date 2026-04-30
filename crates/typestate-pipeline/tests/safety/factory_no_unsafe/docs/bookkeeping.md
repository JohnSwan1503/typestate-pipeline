### Drop-bookkeeping primitives

Per-binary plumbing every Drop-tracking test draws from.

`Counted` is a payload type whose `new` increments `ALIVE` and whose
`Drop` decrements it. Tests that exercise the destructor path assert
`alive() == baseline` before-and-after to prove every value's
`Drop` ran exactly once — leaks and double-drops both fail the
assertion.

`LOCK` serializes Drop-tracking tests against each other within a
single test binary. `cargo test` runs tests inside the same binary
in parallel; without the lock, a `Counted::new` from a concurrent
test would race the baseline snapshot. `serialize()` recovers from
poisoning so a previously-failing test does not cascade-fail every
successor.
