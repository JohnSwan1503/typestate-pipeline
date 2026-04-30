# Factory-bag-in-pipeline integration tests

`#[derive(TypestateFactory)]` and `#[transitions]` target different
shapes of typestate. The factory tracks **within-phase** field
accumulation; the pipeline tracks **cross-phase** progress. They
compose naturally: a phase can hold a fully-set factory bag as its
state, and a transition out of the phase can call `.finalize()` on the
bag to produce the next-phase value.

These tests pin that composition end-to-end:

- `Phase 1 (Drafting)` — state is `UserFactory<Y, Y, Y>` (a fully-set bag).
- `Phase 2 (Submitted)` — state is `User` (the finalized struct).
- `Phase 3 (Confirmed)` — state is `Confirmation` (after async confirmation).

Two transitions:

- `submit` — sync fallible: finalize the bag, validate, advance.
- `confirm` — async deferred: hit the "server", advance.

The happy-path test exercises `submit() -> confirm() -> .await?` as a
single chained expression to confirm the sync-fallible Result folds
into the InFlight chain that `confirm` opens. The validation-failure
test exercises `submit()` returning `Err` to confirm the chain
short-circuits at the bag-finalize step before any pipeline future is
constructed.
