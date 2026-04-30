### Async test helpers

Used by the async-setter cancellation test
([`async_setter_dropped_mid_await_drops_other_set_fields`](../async_setter_dropped_mid_await_drops_other_set_fields/index.html)).

`PendOnce` is a `Future` that returns `Pending` once and then
`Ready` thereafter. Predictable suspension lets the test observe
the partial-drop path at exactly one point.

`poll_once(fut)` builds a no-op waker, polls the future once, then
drops it without resuming. Together they let a test cancel an
async setter without dragging in a runtime — keeping the
regression test about destructor ordering, not about tokio
behavior.
