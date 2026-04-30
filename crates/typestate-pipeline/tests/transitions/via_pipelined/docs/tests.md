# `Pipelined<'a>` integration tests

When a carrier implements `Pipelined<'a>` (via `pipelined!` or
`impl_pipelined!`), `#[transitions]` and
`#[factory(pipeline(carrier = …))]` can both omit `error = …` — the
error type is read from the carrier's `Pipelined::Error` projection,
and destination types come from the GAT projections
(`Resolved<NS>` / `InFlight<NS>`).

This suite pins:

- A `#[transitions]` chain that omits `error =` still compiles and
  runs.
- That chain's errors propagate correctly through the inferred error
  type.
- A `#[derive(TypestateFactory)]` with
  `#[factory(pipeline(carrier = Author))]` (no `error =`) can drive
  setters through the carrier when no fallible setters are declared.
- The `IntoFuture` impl `pipelined!` emits drives an InFlight
  carrier back to its Resolved successor.

Together they prove the GAT-based introspection path is fully
sufficient — users don't need to repeat `error = …` at every
`#[transitions]` site or `#[factory(...)]` site that uses an
existing carrier.
