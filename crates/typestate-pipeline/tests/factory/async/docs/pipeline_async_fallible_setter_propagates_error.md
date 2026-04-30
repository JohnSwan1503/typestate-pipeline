## Pipeline Async Fallible Failure

**Invariant.** When a pipeline-arm async-fallible setter's
transformer rejects, the error surfaces at the chain's terminal
`.await?` — not at the call site. The fallible result folds into
the pending future, mirroring how sync-fallible folds work.

**Failure mode this guards.** The codegen has to wire the async
transformer's `Result` into the chained pending future correctly:

- If the lift was wrong, the call site would force a `Result`
  return (defeating folding).
- If the error mapping was wrong, the surfaced error variant
  could differ from what the transformer produced.

**Setup.** `Author<OrderFactory, Resolved>` from `empty_order`.
Chain: `sku("X")` (async non-fallible, succeeds) → `quantity(0)`
(async fallible, fails because 0 is invalid), terminal `.await`.

**Assertion.** The terminal `.await` returns
`Err(BadInput::Empty)` — exact variant, surfacing through the
folded chain.

### pipeline_async_fallible_setter_propagates_error
