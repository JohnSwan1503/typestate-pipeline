## `impl_pipelined!` — hand-rolling the carrier struct

`pipelined!`'s convenience comes at a cost: it dictates the
struct shape. There's exactly one form — a `'a`-bounded
newtype with a default-`Resolved` mode parameter and a private
inner `Pipeline`. When that shape doesn't fit, reach for
`impl_pipelined!`. It emits the same `Pipelined` impl, the same
`IntoFuture` lift, and the same `inspect` combinator —
everything the trait-driven macros need — but leaves the struct
declaration to you.

Concretely, the cases that prompt this:

- Attaching a `#[derive(Clone)]` when the inner `Pipeline` is
  itself `Clone` (resolved mode plus a clonable state). The
  conventional shape can't carry the derive.
- Pinning extra generics like `<'a, K: Kind, S, M>` so a single
  carrier serves multiple kinds.
- Making the inner field non-private for in-crate convenience.
- Reordering the type parameters or adding extra phantom data
  that the conventional shape doesn't accommodate.

The minimal recipe below shows the trait plumbing in isolation,
applied to a hand-written struct.
