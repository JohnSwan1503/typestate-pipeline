## Async Setter Dropped Mid-Await

**Invariant.** When an `async_fn` setter's returned future is
suspended at an `await` and then dropped (caller cancellation), the
bag captured by the future releases its other set fields cleanly. No
leak.

**Failure mode this guards.** The async setter's transformer runs
inside an `async move { ... }` future. The future captures the bag
and the input value; if the future is dropped before completion, both
captured values must be dropped by the future's auto-cleanup. The
test uses a `PendOnce` future to suspend at exactly one point so the
drop happens deterministically.

This is the safe-mode analog of `factory_no_leak`'s async cancellation
test. The safe-mode codegen has the advantage of not needing
`ManuallyDrop` machinery — Rust's auto-cleanup is enough — but the
test still pins the property in case a future codegen change
introduces a manual move that breaks cancellation semantics.

**Setup.** `AsyncSetterBag` with `other` set, then `main` called with
the async transformer (a no-op that just suspends at `PendOnce` once).
`poll_once(fut)` polls the future once (it suspends at `PendOnce`)
then drops it.

**Assertion.** After the future is dropped, `alive() == baseline` —
both `other` and `m` (the input value moved into the future) were
released by the future's auto-Drop.

### async_setter_dropped_mid_await_drops_other_set_fields
