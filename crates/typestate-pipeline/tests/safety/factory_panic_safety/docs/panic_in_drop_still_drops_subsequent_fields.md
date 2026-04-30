## Panic In Field Drop

**Invariant.** When the bag's manual `Drop` impl walks its fields and
field N's `T::drop` panics, the destructors for fields N+1..end must
still run as the unwind cleans up the bag's stack frame.

**Failure mode this guards.** The pre-fix `Drop` body called
`assume_init_drop` on each set field in declared order. That's a
sequence of statements, and if statement N panicked, statements N+1..
never ran — the `MaybeUninit` slots holding fields N+1..end were left
dangling, never destructed.

The fix reads each set field into its own `Option<T>` stack guard
*before* any user destructor runs. Once each field is in a stack local,
Rust's auto-drop with cleanup-on-panic semantics handles the rest: a
panicking `T::drop` on field N unwinds, but the auto-drops of the
remaining stack locals run during unwind cleanup.

**Setup.** A bag with three required fields — `a: PanickyDrop` first,
then `b` and `c` as `Counted`. After construction, `alive() == 2`.

`drop(bag)` is called inside `catch_unwind`. The bag's manual `Drop`
runs `a`'s destructor first (the one that panics).

**Assertion.** After `catch_unwind` returns `Err`:

- `result.is_err()` confirms the panic happened.
- `alive() == 0` confirms `b` and `c` were dropped during unwind even
  though `a`'s `Drop` panicked first.

### panic_in_drop_still_drops_subsequent_fields
