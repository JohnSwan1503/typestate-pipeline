## Fallible Transformer Success

**Invariant.** A `setter = …, fallible` setter that returns
`Ok(value)` consumes the input bag and yields the next bag with
the flag advanced from `No` to `Yes` and the unwrapped value
stored. Subsequent operations (getter, finalize) see the new value.

**Failure mode this guards.** Two regressions:

1. **Result not unwrapped.** A buggy codegen could store
   `Result<T, E>` in the field slot — type mismatch at finalize.
2. **Flag not advanced on `Ok`.** The setter returns `Ok(bag)` but
   the bag still has the flag at `No`; subsequent getters /
   finalize would fail to compile because they need `Yes`.

**Setup.** `ValidatedUser` with one required field whose setter is
`require_nonempty` (rejects empty strings, otherwise returns
`Ok(value)`). Input is `"Carol"` (non-empty).

**Assertion.**

- The setter returns `Ok(bag)`.
- `bag.name() == "Carol"` (getter works on the `Yes` flag).
- `bag.finalize().name == "Carol"` (the stored value flows
  through finalize).

### fallible_transformer_success
