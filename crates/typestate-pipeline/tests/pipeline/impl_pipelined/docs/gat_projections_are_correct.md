## GAT Projections

**Invariant.** `Pipelined<'a>::Resolved<NS>` and
`Pipelined<'a>::InFlight<NS>` are generic associated types that
project a destination *state* `NS` to the fully-instantiated carrier
type with the appropriate mode. Concretely:

- `Author<'a, Started, Resolved>::Resolved<Finished>` should be
  `Author<'a, Finished, Resolved>` (or anything implementing
  `Pipelined<'a, State = Finished, Mode = Resolved>` with the same
  `Ctx`/`Error`/`Tag`).
- `Author<'a, Started, Resolved>::InFlight<Finished>` should be
  `Author<'a, Finished, InFlight>`.

**Failure mode this guards.** The GAT is what `#[transitions]` uses
to find the destination carrier type when the user writes
`#[transition(into = Finished)]` — the macro emits
`<Self as Pipelined<'a>>::Resolved<Finished>` and lets the trait
resolution find the right concrete type. If the GAT projected to the
wrong type:

- A transition wired to `into = Finished` could compile but produce a
  carrier of the wrong state — silent breakage that wouldn't surface
  until a downstream caller tried to dispatch on the wrong type.
- The macro would fail to find the projection at all (compile error
  with a confusing trait-resolution message).

This test pins the projections by binding generic helpers whose
where-clauses spell out the expected concrete shapes.

**Setup.** Two helper functions — `assert_resolved_projection` and
`assert_inflight_projection` — each constrains a generic carrier `A`
to project its `Resolved<Finished>` / `InFlight<Finished>` to a
carrier with the expected state and mode. The test calls them both
with `Author<'_, Started, Resolved>`.

**Assertion.** Both helpers compile. Compile-time witness only.

### gat_projections_are_correct
