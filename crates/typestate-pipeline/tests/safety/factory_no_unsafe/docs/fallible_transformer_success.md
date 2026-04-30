## Fallible Transformer Success

**Invariant.** A `setter = …, fallible` setter that returns
`Ok(value)` consumes the input bag and yields the next bag with the
flag advanced to `Yes` and the unwrapped value stored.

**Failure mode this guards.** Two parts:

1. The safe-mode codegen has to lift the `Result` correctly: input
   bag → call transformer → on `Ok`, build the new sister-struct shape
   with the unwrapped value in storage and the flag at `Yes`.
2. The `?` short-circuit on the `Ok` branch must reach the new bag
   (not somehow short-circuit through the wrong return value).

**Setup.** `ValidatedUser` with `name` field whose setter is
`require_nonempty` (returns `Err` on empty, `Ok(value)` otherwise).
Input is `"Carol"` (non-empty).

**Assertion.**

- The setter returns `Ok(bag)`.
- `bag.name() == "Carol"` (getter works on the `Yes` flag).
- `bag.finalize().name == "Carol"` (the stored value flows through
  finalize).

### fallible_transformer_success
