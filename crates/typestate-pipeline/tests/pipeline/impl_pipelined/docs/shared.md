### Shared infrastructure

- `Client`, `DummyError` — minimal context and error types. Tests
  don't read or write either; they only need them to satisfy the
  `Pipelined` trait's associated types.
- `Author<'a, S, M>` — hand-rolled carrier newtype around `Pipeline`
  with `impl_pipelined!(Author, ctx = Client, error = DummyError)`
  filling in the trait plumbing. The inner field is `pub` so per-test
  files can construct directly.
- `Tagged<'a, S, M>` — same shape but with a custom `tag = MyTag`
  argument, used to verify the macro accepts the optional tag.
- `Started` / `Finished` — phase-state markers. No data, just type
  parameters for the `State` slot.
- `started_inflight(...)` — convenience constructor for the
  `IntoFuture` test.
