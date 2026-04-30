### Shared infrastructure

- `Counted` / `ALIVE` / `alive()` — drop-bookkeeping primitives. Each
  test that exercises a destructor path asserts `alive() == baseline`
  before-and-after to prove every value's `Drop` ran exactly once.
- `LOCK` / `serialize()` — within-binary serialization of tests that
  share `ALIVE`. `cargo test` runs tests inside the same binary in
  parallel; without the lock, `Counted::new`s from concurrent tests
  would race the baseline snapshot.
- `Reject`, `ValidationError` — minimal `std::error::Error` types
  reused by the fallible-setter / overrider / transformer tests. They
  carry no payload; the tests only care that the error propagates.
- `PendOnce` / `poll_once` — a `Future` that returns `Pending` once
  then `Ready`, plus a hand-roll polling helper. Together they let a
  test suspend an async setter at exactly one point and drop the
  future without resuming, observing the cancellation drop path
  without dragging a runtime in.
