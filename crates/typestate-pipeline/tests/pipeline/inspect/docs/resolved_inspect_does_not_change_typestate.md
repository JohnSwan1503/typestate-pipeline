## Resolved `inspect` Preserves Chain

**Invariant.** The Resolved-arm `inspect` returns `Self` — the same
type the chain entered with. A subsequent `.tag()` call (which is
defined on `Author<Drafted>`) must still typecheck after the
`inspect`.

**Failure mode this guards.** A buggy codegen could change the
return type on the Resolved arm — e.g. drop the lifetime, change
the state generic, or wrap in another type. Any of those would
break downstream chains:

```text
pipeline
    .inspect(|c| ...)   // suppose this returned `Author<???, _>`
    .tag()              // .tag() defined on `Drafted` — wrong type → compile error
```

The test exercises exactly this: `inspect(...).tag().await?` must
typecheck. If the typestate changed, the test wouldn't compile.

**Setup.** Fresh `Author<Drafted, Resolved>`. Call
`inspect(...).tag().await.unwrap()`.

**Assertion.** The chain compiles. The resulting carrier has
`name == "beta"` and `tag == 1` (first id from `Hub`). The closure
ran (an `assert_eq!` inside it would have panicked if not).

### resolved_inspect_does_not_change_typestate
