## Fallible Transformer Failure

**Invariant.** A `setter = …, fallible` setter that returns `Err`
propagates the error to the caller and does *not* advance the bag
state. The original bag is consumed by the setter (per the move
semantics), but no flag advances and the failing input is dropped
inside the transformer.

**Failure mode this guards.** A buggy codegen could:

- Swallow the error and silently advance the flag with whatever
  garbage the transformer left in storage.
- Move forward with an empty value if the transformer dropped the
  input but signal `Ok`.

**Setup.** `ValidatedUser` whose setter rejects empty input.
`name(String::new())` is called.

**Assertion.** The setter returns `Err(ValidationError("name is empty"))`.
The exact error message is matched, not just `Err(_)` — pinning the
error contents.

### fallible_transformer_failure
