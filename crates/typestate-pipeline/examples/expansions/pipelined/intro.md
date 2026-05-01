## `pipelined!` — declaring carriers in one line

`#[transitions]` operates on a *carrier*. The carrier is the
type that holds the current phase, threads the context and
error type through every transition, and switches between the
synchronous `Resolved` mode and the future-holding `InFlight`
mode the deferred fold relies on. Every codebase using
`#[transitions]` needs one.

You can write the carrier by hand. It's not difficult; it's
just the same boilerplate every time — a tuple-struct newtype
with a fixed shape, a [`Pipelined<'a>`](crate::Pipelined) impl
that names the context and error and projects the GAT
successors `#[transitions]` consumes to compute the next phase's
type, and an `IntoFuture` lift so an `InFlight` carrier awaits
back into a `Resolved` one. Three impl blocks, mostly empty.

`pipelined!` collapses all of it into a single line:

```rust,ignore
typestate_pipeline::pipelined!(pub Author, ctx = Hub, error = AppError);
```

The expansion produces the newtype `Author<'a, S, M = Resolved>`,
the `Pipelined` impl, the `IntoFuture` lift, and a chainable
`inspect(|c| …)` combinator on both modes. The two sections below
cover the baseline and the optional phantom tag that
distinguishes otherwise-identical carriers at the type level.

When the conventional shape doesn't fit — custom derives, extra
generics, unusual ordering — `impl_pipelined!` (covered after
this section) emits the same trait plumbing without declaring
the struct.
