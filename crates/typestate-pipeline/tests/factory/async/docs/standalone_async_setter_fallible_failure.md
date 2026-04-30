## Async Fallible Setter Failure

**Invariant.** When an `async_fn, fallible` setter's transformer
returns `Err`, the awaited result is `Err(Error)` — exactly like a
sync fallible setter, just behind an `.await`.

**Failure mode this guards.** Two failure shapes:

- The codegen's "lift future + bind `?` inside" path could lose the
  error variant, surfacing a default error instead of the one the
  transformer produced.
- The error type could be inferred wrong (e.g. `Box<dyn Error>`
  instead of the carrier's `BadInput`), forcing the call site to
  use `?` differently.

**Setup.** `UserProfile` with `name` set to "Bob". Then
`email(String::new()).await` triggers the
`validate_email_async`'s `Err(BadInput::Empty)` branch.

**Assertion.** `result` matches `Err(BadInput::Empty)` exactly —
not `Err(_)`. Pinning the variant ensures the error type didn't
get type-erased.

### standalone_async_setter_fallible_failure
