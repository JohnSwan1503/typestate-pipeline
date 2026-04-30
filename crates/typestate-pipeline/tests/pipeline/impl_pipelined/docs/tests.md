# `impl_pipelined!` smoke tests

Verifies that `impl_pipelined!` emits a usable `Pipelined<'a>` impl
and `IntoFuture` forwarding for the conventional carrier shape. The
end-to-end exercise of the trait against `#[transitions]` (with
`error =` omitted) lives in
[`tests::transitions::via_pipelined`](../../transitions/via_pipelined/index.html);
this suite is the type-system / single-trait verification.

Four properties pinned:

- `Pipelined<'a>::Ctx`, `::Error`, `::Tag` resolve to the declared
  types on both Resolved and InFlight modes.
- `Pipelined<'a>::Resolved<NS>` and `::InFlight<NS>` GAT projections
  yield carriers of the expected concrete type.
- The `IntoFuture` impl drives an InFlight carrier back to its
  Resolved successor.
- Customizing the `tag = MyTag` argument also works (tagged carrier
  satisfies `Pipelined<'a, Tag = MyTag>`).

The carrier struct is hand-rolled (not declared via `pipelined!`), so
this also acts as a smoke test of the orphan-rule pattern that
`impl_pipelined!` supports — the user owns the struct, the macro
fills in the trait plumbing.
