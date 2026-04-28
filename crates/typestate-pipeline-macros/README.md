# typestate-pipeline-macros

Proc-macros for the [`typestate-pipeline`] ecosystem:

- `#[derive(TypestateFactory)]` — named-field accumulator with one
  flag generic per field; setters flip `No` → `Yes` and `finalize()`
  is callable only when every required flag is `Yes`.
- `#[transitions]` — generates Resolved + InFlight method pairs from a
  single source body on a `Pipeline` newtype, reading destination
  types off the carrier's `Pipelined<'a>` impl.

**Use [`typestate-pipeline`] instead of depending on this crate
directly.** The generated code emits fully-qualified paths through
`::typestate_pipeline::__private::*`; without the façade those paths
do not resolve. The façade re-exports both macros and adds the
`pipelined!` / `impl_pipelined!` declarative shorthands for the
conventional carrier shape.

See the [main README] for the full attribute reference, examples, and
safety notes.

## License

Licensed under either of Apache-2.0 or MIT at your option.

[`typestate-pipeline`]: https://crates.io/crates/typestate-pipeline
[main README]: https://github.com/JohnSwan1503/typestate-pipeline#readme
