### Async deferred: folding the chain

This is the shape that justifies the whole two-arm scheme.

A typical async pipeline has multiple awaitable steps, and the
naive ergonomic costs the caller an `.await?` after every link.
The deferred-async transition flips that. Both arms produce
`InFlight` carriers, so a chain of async transitions folds into
*one* `IntoFuture` driven by *one* terminal `.await?` at the end:

```rust,ignore
let deployed = author.tag_version(7).deploy().confirm().await?;
//                   InFlight    ↑   InFlight  ↑  InFlight  ↑  Resolved
//                   (no .await per step — only the last one drives)
```

The body is `async fn` returning `Result<Next, E>`, with no
`breakpoint` on the `#[transition]` attribute (deferred is the
default for `async fn` bodies).

- The **Resolved arm** wraps the body's future and lifts the
  carrier from `Resolved` to `InFlight`. A previously-eager
  chain joins the async fold here.
- The **InFlight arm** chains the body's future onto the
  pending future; the result is `InFlight` again. Every link
  after this one stays in the fold.

The Resolved-arm lift is what makes mixing easy: the caller
doesn't have to think about which transitions are sync and
which are async. The first async transition in a chain
auto-lifts; sync transitions after it stay in the fold via the
InFlight arms covered earlier; everything resolves at the
single terminal `.await?`.

When the next link's *arguments* depend on the resolved value
of this one — when computing them requires reading the just-
produced state — the deferred fold doesn't fit. That's the
next recipe.
