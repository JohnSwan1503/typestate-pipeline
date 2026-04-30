## Defaults at `finalize`

**Invariant.** A bag with multiple optional-with-default fields,
none of which were explicitly set, finalizes with each field equal
to its declared default expression's value.

**Failure mode this guards.** The unsafe-mode `finalize` reads each
flag's `IS_SET` constant at runtime: if `Yes`, `assume_init_read`
the slot; if `No`, evaluate the default expression. A buggy codegen
could:

- Evaluate the default for every field (overwriting explicit
  values).
- Read the slot for every field (UB on uninitialized memory when
  flag is `No`).

**Setup.** `Configurable` with `name` (required), `parallelism`
(default 8), `url` (default
`"https://default.example".to_owned()`). Only `name` is set.

**Assertion.** All three default values appear in the finalized
struct.

### finalize_uses_defaults_when_optional_no
