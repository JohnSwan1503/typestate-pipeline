## `#[transitions]` — advancing through phases

The factory builds a *value*; `#[transitions]` advances a value
through *phases*. The two macros use the same idea — one
`Yes`/`No` flag per dimension that decides what compiles — but
applied to different domains. Where the factory tracks "which
fields are set," `#[transitions]` tracks "which phase we're in,"
and the phase becomes the carrier's state-type generic.

The macro decorates a plain `impl` block on a typestate carrier.
Each method marked `#[transition(into = NextPhase)]` is rewritten
into *two* method arms behind a single declaration: one for
`Resolved` mode (the carrier holds a real value, so the body
runs immediately) and one for `InFlight` mode (the carrier holds
a pending future, so the body chains onto it).

Without that two-arm symmetry, an async chain of three steps
costs the caller three `.await?`s and three `let`-bindings:

```rust,ignore
let versioned = author.tag_version(7).await?;
let queued    = versioned.deploy().await?;
let confirmed = queued.confirm().await?;
```

Every break is a place to drop a value, mistype a name, or
refactor in a bug. With both arms emitted, the chain folds
into a single terminal `.await?` regardless of how many links
are async:

```rust,ignore
let confirmed = author.tag_version(7).deploy().confirm().await?;
```

That is what the symmetry buys.

Four body shapes cover every case the macro recognizes:
sync infallible, sync fallible, async deferred, and async
breakpoint. They're the next four sections.
