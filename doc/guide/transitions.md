# `#[transitions]` and `pipelined!`

## `#[transitions]`

Decorates an `impl` block on a tuple-struct newtype around `Pipeline`.
Each method marked `#[transition(into = NextState)]` is expanded into a
Resolved + InFlight method pair from a single source body. Four body
shapes are recognized: sync infallible, sync fallible, async deferred
(default for `async fn`), and async breakpoint (`breakpoint`).

```rust,ignore
use typestate_pipeline::{pipelined, transitions};

pipelined!(Author, ctx = Client, error = AuthoringError);

#[transitions]
impl<'a> Author<'a, Registered> {
    #[transition(into = Versioned)]
    pub async fn tag_version(state: Registered, ctx: &Client, version: u32)
        -> Result<Versioned, AuthoringError>
    {
        ctx.tag(state.name.clone(), version).await;
        Ok(Versioned { name: state.name, version })
    }
}

// chain folds into a single terminal `.await?`
let v = author.tag_version(7).deploy().await?;
```

The destination type is read off the carrier's `Pipelined<'a>` impl as a
GAT projection (`<Self as Pipelined<'a>>::Resolved<NextState>`), so
carriers with extra generics or unusual ordering keep working as long as
the trait impl is correct. Generated transition code uses **no** `unsafe`.

## `pipelined!` / `impl_pipelined!`

Declarative shorthand for the conventional carrier shape
(`<'a, S, M = Resolved>` tuple-struct newtype around `Pipeline`):

```rust,ignore
// declares the carrier struct + Pipelined impl + IntoFuture forwarding
typestate_pipeline::pipelined!(pub Author, ctx = Client, error = AuthoringError);

// alternative: hand-write the struct (custom derives, extra generics, …)
// and emit only the trait impls
typestate_pipeline::impl_pipelined!(Author, ctx = Client, error = AuthoringError);
```

Both also emit a chainable `inspect(|carrier| …)` combinator on Resolved
and InFlight.
