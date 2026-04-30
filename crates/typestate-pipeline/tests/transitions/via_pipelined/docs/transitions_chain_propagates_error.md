## Inferred Error Propagation

**Invariant.** When `#[transitions]` reads the error type from
`Pipelined::Error`, errors from the body bubble out through the
chain just as they would with an explicit `error = AppError`. The
inferred path isn't lossy.

**Failure mode this guards.** A subtle codegen bug could pick up
the right error type but emit the wrong `From` / `?` boundaries —
the body's `Err(AppError::Bad(...))` would then be wrapped in some
other type or dropped silently. The test pattern-matches on the
exact variant to surface this.

**Setup.** Same `Author` / `Drafted` / `Versioned` / `Published`
plumbing. Call `tag(0)` to trigger the body's `Err(AppError::Bad("version must be > 0"))`.

**Assertion.** `result` is `Err(AppError::Bad(m))` where
`m == "version must be > 0"` — exact variant and exact message.

### transitions_chain_propagates_error
