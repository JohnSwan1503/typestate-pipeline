### Drop-bookkeeping primitives

Per-binary plumbing every panic-safety test draws from.

- `COUNTED_ALIVE` / `alive()` — atomic counter incremented on
  `Counted::new` and decremented on `Counted::drop`. Tests assert
  the counter goes to zero after `catch_unwind` settles, proving
  every field's destructor ran on unwind.
- `PANICKY_FUSE` — one-shot ammunition for `PanickyDrop`. Without
  it, the override / remove tests would double-panic during unwind
  (the *old* value panics, then the *new* value's `Drop` would
  panic too). One-shot keeps the second `Drop` quiet so the unwind
  completes.
- `setup()` — resets both counters and arms the fuse, plus locks
  `LOCK` to serialize against peer tests in the same binary. The
  lock recovers from poisoning so a previously-failing test
  doesn't cascade-fail every successor.
- `Counted(label)` — alive-counter payload type.
