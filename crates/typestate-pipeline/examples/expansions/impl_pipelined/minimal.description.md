### Minimal: trait plumbing without the struct

`impl_pipelined!(Author, ctx = Hub, error = AppError)` emits the
`Pipelined` impl, the `IntoFuture` lift, and the `inspect`
combinator — exactly the trait-driven surface the macros need —
and *nothing else*. The struct is your responsibility.

The cases that prompt this:

- Attaching a custom `#[derive(Clone)]` when the inner
  `Pipeline` is `Clone` (resolved mode + clonable state).
- Pinning extra generics — `<'a, K: Kind, S, M>` — that the
  conventional shape can't express.
- Making the inner field non-private for in-crate convenience.
- Non-default field ordering, additional phantom data, or any
  other shape constraint the `pipelined!` form forbids.

The arguments are otherwise identical to `pipelined!`: same
`ctx`, `error`, optional `tag`. Only the struct declaration
differs.
