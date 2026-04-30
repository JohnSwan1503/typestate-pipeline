### Async breakpoint: pausing the chain

The deferred fold works because no link reads the *value* of
the previous one — every link only knows the previous link's
*type*. When the next argument value depends on the just-
produced state — computing a version bump from the current
version, matching on a phase to dispatch the next transition,
deciding *which* downstream call to make — the fold has to
break.

`#[transition(into = Next, breakpoint)]` does exactly that.
Both arms keep the method `async` and resolve to `Resolved`
mode, forcing the caller to `.await?` (or otherwise drive the
future) here, so the next link sees the resolved value:

```rust,ignore
let versioned = author.confirm_and_tag().await?; // breakpoint
let deployed  = versioned.deploy(versioned.version + 1).await?;
//                                ^^^^^^^^^^^^^^^^^^^^^
//                                needs the resolved value
```

A breakpoint costs the chain its single-`.await?` ergonomic
for that link. Use it deliberately — at the boundaries where
the next argument actually depends on the resolved state — not
as a default. The deferred fold is shorter and reads better
when the chain doesn't need to peek at intermediate values.
