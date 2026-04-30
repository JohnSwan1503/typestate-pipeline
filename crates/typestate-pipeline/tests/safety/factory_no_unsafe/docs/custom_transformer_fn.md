## Sync Transformer

**Invariant.** A `setter = my_fn` attribute composes with safe-mode
storage: the setter runs the transformer on its input and stores the
return value into the bag's `Yes` slot. The setter signature is
identical to baseline; only the body differs.

**Failure mode this guards.** A safe-mode codegen that bypassed the
transformer (writing the raw input into storage) would silently break
every user that relies on normalization (trim, lowercase, parse).
Easy to introduce because the safe-mode path has its own setter
template; if the template doesn't include the transformer call, the
input flows directly to storage and the test asserts on the
non-transformed value.

**Setup.** `NormalizedUser` with one required field whose setter
routes through `trim_name(value: String) -> String`. Input has
leading/trailing whitespace.

**Assertion.** `u.name == "Bob"` — whitespace removed by the
transformer, not the original `"   Bob   "`.

### custom_transformer_fn
