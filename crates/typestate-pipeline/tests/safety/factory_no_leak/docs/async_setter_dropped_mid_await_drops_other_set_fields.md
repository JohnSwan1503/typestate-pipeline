## Async Setter Dropped Mid Await

**Invariant.** When an `async_fn` setter's returned future is dropped
mid-`await` (i.e. the caller cancels), the bag captured by that future
must release every previously-set field. Cancellation must be safe — a
future that never completes shouldn't leave value-payloads alive.

**Failure mode this guards.** This is the same `ManuallyDrop` leak as
in the sync-fallible case, but it bites *without* an explicit `?`. The
pre-fix codegen for `async_fn` setters wrapped `self` in `ManuallyDrop`
*before* the `await`; if the future was dropped at a suspension point,
the inner `MaybeUninit` slots holding the bag's already-set fields were
never touched. The fix moves the `ManuallyDrop` wrap to *after* the
transformer awaits.

The async test exposes the bug independently of `?` propagation —
proving the fix isn't just about the `Result` short-circuit path but
about the more general "transformer ran before `ManuallyDrop`" ordering.

**Setup.**

- `AsyncSetterBag` — two required `Counted` fields. `main`'s setter
  routes through `async_pending`, an `async fn` that suspends once via
  `PendOnce` before yielding its argument unchanged.
- `other` is set first; `alive() == baseline + 1`.
- `bag.main(Counted::new("m"))` returns a future that captures `bag`
  (carrying `other`) and `m`. After moving them in, `alive() == baseline + 2`.
- `poll_once(fut)` polls the future once — it suspends at `PendOnce` —
  then drops the future without resuming.

**Assertion.** After the future is dropped at its suspension point,
`alive()` is back to `baseline`. With the buggy codegen, `other` would
remain alive because dropping a `ManuallyDrop`-wrapped `self` skips its
`Drop`.

**Why hand-poll instead of an executor.** The test never wires the
future to a runtime; it just wants a single `Poll::Pending` and then
a controlled drop. `Waker::noop()` and a manual `Box::pin` give us the
suspension/drop sequence without dragging tokio into a regression test
that's about destructor ordering, not async runtime behavior.

### async_setter_dropped_mid_await_drops_other_set_fields
