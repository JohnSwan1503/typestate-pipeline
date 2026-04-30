# Testing

Tests are grouped by domain — each `tests/<domain>.rs` is the integration
binary, and the actual cases live as child modules under `tests/<domain>/`.

The full per-test coverage tables live in the
[README on GitHub](https://github.com/JohnSwan1503/typestate-pipeline#testing).
This page summarizes what each domain pins down and links straight to
the integration suite.

## [`factory`] — `#[derive(TypestateFactory)]` feature coverage

[`factory`]: https://github.com/JohnSwan1503/typestate-pipeline/tree/main/crates/typestate-pipeline/tests/factory

- `core.rs` — declared-order vs. arbitrary-order setters, `<field>_default()`
  helper, getter borrowing, partial-bag and fully-set-bag drop semantics,
  `finalize` not double-dropping, `drop_<field>` and `override_<field>`
  drop semantics, defaults at finalize, `setter = <fn>` transformers,
  fallible setters (`Ok`/`Err` paths), custom bag and setter names.
- `async.rs` — `async_fn` setters (success and failure), `finalize_async`,
  pipeline-arm async setters threading through `InFlight`, transition
  bodies that call `.finalize()` mid-chain.
- `input_type.rs` — `input = …` setter input types and how they bridge
  to the storage type via the transformer.
- `internal_field.rs` — internals set positionally on `new(…)`, dropped
  from the flag-generic list, with unconditional getters on both bag and
  carrier arms.
- `ready_trait.rs` — `<Bag>Ready` is implemented as soon as every
  required flag is `Yes`; generic `B: <Bag>Ready` dispatch matches
  inherent `finalize()`.

## [`transitions`] — `#[transitions]` body shapes and `Pipelined` resolution

[`transitions`]: https://github.com/JohnSwan1503/typestate-pipeline/tree/main/crates/typestate-pipeline/tests/transitions

- `core.rs` — full Resolved → InFlight → Resolved chain folding into a
  single terminal `.await?`, `breakpoint` breakpoints forcing
  explicit awaits, sync fallible Resolved arm vs. sync fallible InFlight
  arm, `IntoFuture` driving InFlight back to Resolved.
- `via_pipelined.rs` — chains with and without an `error =` arg,
  factory pipeline arms with and without an error type, and the
  `Pipelined`-supplied `IntoFuture`.
- `attr_forwarding.rs` — attributes on the source `impl` block forwarded
  to both generated arms.

## [`integration`] — cross-feature scenarios

[`integration`]: https://github.com/JohnSwan1503/typestate-pipeline/tree/main/crates/typestate-pipeline/tests/integration

- `factory_pipeline.rs` — bag setters reaching the carrier in Resolved
  mode, threading through InFlight, `drop_<field>` flipping the
  carrier-side flag, `override_<field>` replacing the bag value via the
  carrier arm.
- `factory_in_pipeline.rs` — standalone bag's `finalize()` feeding a
  pipeline transition mid-chain; bag-level fallible finalize
  short-circuiting the surrounding chain.
- `dataset_authoring.rs` — the full multi-phase authoring pipeline
  (gated on `dataset-authoring-example`); patch-bump arithmetic; typed
  errors on kind mismatch.

## [`pipeline`] — dual-mode `Pipeline` carrier and `inspect` combinator

[`pipeline`]: https://github.com/JohnSwan1503/typestate-pipeline/tree/main/crates/typestate-pipeline/tests/pipeline

- `impl_pipelined.rs` — `Pipelined` associated types and GAT projections
  resolve correctly, `IntoFuture` on InFlight awaits to a Resolved
  successor, carriers with extra generics still satisfy `Pipelined`.
- `inspect.rs` — `inspect(|carrier| …)` runs synchronously on Resolved,
  preserves the chain, and on InFlight runs only after the pending
  future resolves while keeping the chain in InFlight.

## [`ui`] — trybuild compile-fail diagnostics

[`ui`]: https://github.com/JohnSwan1503/typestate-pipeline/tree/main/crates/typestate-pipeline/tests/ui

`tests/ui_macros.rs` drives [trybuild](https://docs.rs/trybuild) over
`tests/ui/*.rs` to pin the wording of macro diagnostics for misuse —
wrong attribute combinations, unset required fields, attempting to read
private `Pipeline` fields, and so on.
