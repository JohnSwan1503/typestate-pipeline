# Mental model

Two orthogonal axes. Each macro operates on one of them, and they
compose freely.

## Workspace layout

```text
typestate-pipeline          # facade — depend on this
├── typestate-pipeline-core    # runtime: Pipeline, Mode, Pipelined, flag traits
└── typestate-pipeline-macros  # proc-macros (use through the facade)
```

The proc-macros emit fully-qualified paths through
`::typestate_pipeline::__private::*`, so always depend on the facade
crate; depending on the macros crate alone produces unresolved paths.

## Factory/Builder mode — `#[derive(TypestateFactory)]`

Every non-`internal` field on a derived bag carries a flag generic that is
either `No` (unset) or `Yes` (set). Setters flip `No → Yes`; `finalize()`
resolves only when every required flag is `Yes`. The full transition
graph, covering every `#[field(…)]` mutability attribute:

- `Factory::new()` / `Default::default()` — initial state; every flag `No`.
- `.field(val)` setter (required field, default naming) — `No → Yes`.
- `.with_field(val)` setter (`optional` or `default`) — `No → Yes`.
- `.field_default()` helper (`default = …`) — `No → Yes`, using the
  declared default expression.
- `.drop_field()` (`removable`) — `Yes → No`; drops the stored value.
- `.override_field(val)` (`overridable`) — `Yes → Yes`; drops the old
  value before storing the new one.
- `.finalize()` — consumes the bag once every required flag is `Yes`
  (optional-with-default flags may be either).

`#[field(internal)]` fields don't appear in the flag-generic list at all
— they're set positionally on `new(…)` and have an unconditional getter.

## Carrier mode — `#[transitions]`, `pipelined!`, `impl_pipelined!`

The pipeline carrier is dual-mode: `Resolved` holds the current state
directly, `InFlight` holds a pending future that will yield the next
state. Each `#[transition]` body shape picks which arrow it takes; the
two arms emitted per transition (one per starting mode) end up in
different places:

- **Sync infallible** (`fn` returning a non-`Result`):
  `Resolved → Resolved`, `InFlight → InFlight`.
- **Sync fallible** (`fn` returning `Result<_, E>`):
  `Resolved → Result<Resolved, E>` (handle at the call site),
  `InFlight → InFlight` (the `Result` folds into the pending future).
- **Async deferred** (`async fn`, the default for async bodies):
  `Resolved → InFlight` (lifts the chain), `InFlight → InFlight`.
- **Async breakpoint** (`#[transition(breakpoint)]`): both arms
  return `async fn → Result<Resolved, E>`; the caller must `.await?` here.

Crosscutting: any `InFlight` carrier `.await?`s into a `Resolved` of the
same state via the carrier's `IntoFuture` impl.

## Chain folding

A run of async-deferred transitions stays in `InFlight` for the whole
chain and resolves at a single terminal `.await?`:

```rust,ignore
Author<Registered, Resolved>    // start
    .tag_version(7)             // async deferred  -> lifts to InFlight
    .with_parallelism(8)        // sync infallible -> folds into pending
    .deploy()                   // async deferred  -> folds into pending
    .await?                     //                 -> resolves to Author<Deployed, Resolved>
```

Adding `#[transition(into = …, breakpoint)]` to one of the steps
(an *async breakpoint*) forces the chain to `.await?` at that step,
landing back in `Resolved` for whatever follows.
