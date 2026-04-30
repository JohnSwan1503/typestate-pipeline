## Explicit Optional at `finalize`

**Invariant.** When an optional-with-default field is explicitly set
via `with_<field>(...)`, `finalize` reads the stored value, not the
default expression.

**Failure mode this guards.** The `Storage::finalize_or` dispatch has
to pick the right branch based on the flag at the call site:

- `Yes` flag → return the stored `T`.
- `No` flag → evaluate the default thunk.

A buggy `Storage<T> for Yes` impl that returned the default thunk's
value (instead of the stored value) would silently overwrite explicit
user input with defaults — a particularly nasty failure because tests
often assert on default values, not explicit ones.

**Setup.** `Configurable` with all three fields set: `name`,
`with_parallelism(16)`, `with_url("https://override.example".to_owned())`.

**Assertion.** `parallelism == 16` and `url == "https://override.example"`
— the explicit values, not the declared defaults.

### finalize_keeps_explicit_values_when_optional_yes
