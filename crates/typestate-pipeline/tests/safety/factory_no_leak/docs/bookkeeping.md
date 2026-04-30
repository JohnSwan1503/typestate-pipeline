### Drop-bookkeeping primitives

Per-binary plumbing every test file in this suite shares.

`Counted` is a payload type that increments a global atomic on construction
and decrements it on `Drop`. Tests assert `alive() == baseline` to prove
that every `Counted` they constructed has run its destructor exactly
once — leaks and double-drops both fail the assertion.

`LOCK` serializes tests against each other within a single test binary.
`cargo test` runs tests inside the same binary in parallel by default;
without the lock, an unrelated test creating its own `Counted`s would
race the `assert_eq!(alive(), baseline)` snapshot and produce flaky
green-on-red results. `serialize()` recovers from poisoning so a
previously-failing test does not cascade-fail every successor.
