## Custom Transformer

**Invariant.** `setter = my_fn` runs `my_fn(val)` inside the setter
and stores the return value. The setter's *input* type is still
the field's storage type (use `input = T` for a different input —
see [`tests::factory::input_type`](../../input_type/index.html)).

**Failure mode this guards.** A buggy codegen could bypass the
transformer and store the raw input — `name == "   Bob   "` (with
whitespace) instead of `name == "Bob"` (trimmed).

**Setup.** `NormalizedUser` with one required field whose setter
routes through `trim_name(value: String) -> String`. Input has
leading/trailing whitespace.

**Assertion.** `u.name == "Bob"` — whitespace removed by the
transformer.

### custom_transformer_fn
