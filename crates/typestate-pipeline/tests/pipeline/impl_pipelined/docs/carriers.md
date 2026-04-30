### Carriers

Two hand-rolled carriers — `impl_pipelined!`'s job is to fill in
the trait plumbing without declaring the struct.

- `Author<'a, S, M>` exercises the default-tag (`Tag = ()`) shape.
- `Tagged<'a, S, M>` exercises the custom `tag = MyTag` arg.

Both have `pub` inner fields so per-test files can construct them
directly without crossing the `pipelined!` private-field
boundary.

`started_inflight(client, pending)` is a convenience wrapper for
the `IntoFuture` test, building an InFlight `Author` from a
hand-built future.

`Client` is the context type — opaque, no behaviour; tests only
need it to satisfy `Pipelined::Ctx = Client`.
