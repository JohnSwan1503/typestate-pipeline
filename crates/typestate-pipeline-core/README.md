# typestate-pipeline-core

Runtime primitives for the [`typestate-pipeline`] ecosystem: the
`Pipeline<'a, Ctx, Tag, S, E, M>` carrier and its `Mode` / `Resolved` /
`InFlight` machinery, the `Pipelined<'a>` marker trait the proc-macros
introspect to find a carrier's destination types and error, and the
`Yes` / `No` / `Satisfiable` / `Satisfied` flag traits used by
`#[derive(TypestateFactory)]`.

You almost certainly want the façade crate [`typestate-pipeline`]
instead — it re-exports everything here alongside the proc-macros and
the `pipelined!` / `impl_pipelined!` declarative shorthands. The
proc-macros emit fully-qualified paths through
`::typestate_pipeline::__private::*`, so depending on this crate alone
will not let you use them.

## License

Licensed under either of Apache-2.0 or MIT at your option.

[`typestate-pipeline`]: https://crates.io/crates/typestate-pipeline
