## Pipeline Async Setter

**Invariant.** With `#[factory(pipeline(carrier = Author))]` and
`async_fn` setters, the pipeline-arm setter on a Resolved carrier
returns InFlight (lifts the chain). Subsequent pipeline-arm setters
fold into the same pending future. One terminal `.await?` drives
the whole chain.

**Failure mode this guards.** Without correct lifting, the chain
would fragment: each async setter would force its own `.await`,
defeating chain-folding. The Resolved-arm signature for an
async-fallible setter must be
`fn(self, val) -> Author<NextBag, InFlight>` (lifts), not
`async fn(self, val) -> Result<Author<NextBag, Resolved>, E>`
(breakpoint shape).

**Setup.** `Author<OrderFactory, Resolved>` from `empty_order`.
Chain: `sku("  SKU-42  ")` (async non-fallible, lifts) →
`quantity(5)` (async fallible, folds), terminal `.await?`.

**Assertion.** Awaiting the chain yields a Resolved bag whose
`finalize` produces an `Order` with `sku == "SKU-42"` (trimmed)
and `quantity == 5` (validated as non-zero).

### pipeline_async_setter_chains_through_inflight
