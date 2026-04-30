# `inspect` combinator tests

`pipelined!` and `impl_pipelined!` both emit a chainable `inspect(|c|
…)` combinator on Resolved *and* InFlight modes. It runs a closure
against the carrier (or, for InFlight, the eventual resolved carrier)
without changing the typestate, so the chain continues unchanged.

Two arms tested:

- **Resolved.** Closure runs synchronously on the carrier; carrier is
  returned unchanged so the chain continues. Two tests: one observes
  the closure ran, one verifies the typestate is unchanged so a
  follow-on `.tag().await?` still typechecks.
- **InFlight (deferred).** Closure runs *after* the pending future
  resolves, against a temporary `Resolved` carrier reference. The
  chain returns to `InFlight` so subsequent transitions keep folding
  into a single terminal `.await?`. Two tests: one verifies the
  closure runs only after `.await`, one verifies the chain folds into
  a downstream `.deploy()` call.
