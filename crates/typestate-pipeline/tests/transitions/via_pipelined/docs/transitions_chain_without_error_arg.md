## Inferred Error Type

**Invariant.** A `#[transitions]` impl block that omits `error = …`
reads the error type from `<Self as Pipelined<'a>>::Error`. The
chain compiles and runs identically to one that spells out the
error type.

**Failure mode this guards.** A regression in the macro's GAT
introspection path could:

- Fail to find `<Self>::Error` and emit a wrong type bound (compile
  error).
- Pick up the wrong type (e.g. `()`) and accept an unrelated body
  signature, then fail at chain composition.
- Emit a fallback error type that diverges from what `pipelined!`
  declared, causing the chain to wrap the wrong error variant.

**Setup.** `Author` declared with `pipelined!(... error = AppError)`.
A `#[transitions]` impl on `Author<Drafted>` declares `tag` (async
deferred) without an `error =` arg. A second impl on
`Author<Versioned>` declares `publish` (sync fallible) similarly.

**Assertion.** `initial.tag(7).publish().await.expect(...)` succeeds
and yields a `Published` with `name == "alpha"` and `version == 7`.

### transitions_chain_without_error_arg
