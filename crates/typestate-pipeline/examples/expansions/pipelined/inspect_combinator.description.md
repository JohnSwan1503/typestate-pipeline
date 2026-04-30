### Inspect: side-effects without state changes

The carrier is the right place to attach observability —
logging, tracing, span attribution — because it sees every
state advance. But the typestate is unforgiving: a closure
that wants to peek at the carrier without changing the phase
has to *return* the carrier in exactly the shape it received.
Doing that by hand is annoying enough to discourage tracing.

`pipelined!` and `impl_pipelined!` both emit a chainable
`inspect(|c| …)` combinator on Resolved *and* InFlight modes
to handle this exactly. The closure runs against the carrier
(or, for InFlight, the eventual resolved carrier), and the
chain continues unchanged.

- The **Resolved arm**: `inspect(F: FnOnce(&Self))` — the
  closure runs synchronously against the resolved carrier; the
  same carrier is returned.
- The **InFlight arm**:
  `inspect(F: FnOnce(&Author<'a, S, Resolved>) + Send + 'a)` —
  the closure runs *after* the pending future resolves, against
  a temporary `Resolved` view; the carrier is rewrapped as
  `InFlight` so subsequent transitions keep folding.

The InFlight arm's `Send + 'a` bound is the price of the fold:
the closure becomes part of the pending future, and that
future has to outlive the carrier's lifetime and cross await
points safely.
