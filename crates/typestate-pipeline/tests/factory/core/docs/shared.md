### Shared infrastructure

- `Counted` / `ALIVE` / `alive()` — drop-bookkeeping primitives. The
  Drop tests assert `alive() == baseline` before-and-after to prove
  every value's `Drop` ran exactly once.
- `LOCK` / `serialize()` — within-binary serialization of tests
  that share `ALIVE`. `cargo test` runs tests inside the same binary
  in parallel; without the lock, `Counted::new` calls from
  concurrent tests would race the baseline snapshot. `serialize()`
  recovers from poisoning so a previously-failing test doesn't
  cascade-fail every successor.
