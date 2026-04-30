## Setter Input Type

**Invariant.** `with_worker(...)` accepts `String` (the declared
`input` type) and the `wrap_some` transformer lifts the value into
the field's storage type `Option<String>`. The transformer runs
inside the setter; the user never writes `Some(...)` at the call
site.

**Failure mode this guards.** A buggy codegen could:

- Make the setter take the storage type (`Option<String>`),
  defeating the `input = ...` configuration.
- Skip the transformer entirely, storing the input directly into the
  storage slot — that would be a type mismatch and refuse to compile.

**Setup.** A `Profile` bag with `name` (required) and `worker`
(`Option<String>` storage, `String` input). Setter chain:
`name(...).with_worker("worker-1".to_owned())`.

**Assertion.** `profile.worker == Some("worker-1")` — the
transformer ran and lifted the input.

### setter_takes_input_type_not_field_type
