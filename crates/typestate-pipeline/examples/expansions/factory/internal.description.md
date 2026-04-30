### Internal field: locked at construction

Every recipe so far has assumed each field can be set, unset,
or left to a default — that the caller has agency over every
field. Sometimes that's wrong. A namespace identifier, a cluster
key, a session token: these are "given context" the constructor
needs but no caller should be able to overwrite later.

`#[field(internal)]` models exactly that. Four user-visible
consequences:

- `new(…)` takes the internal field as a parameter — it's
  positional, supplied at construction.
- The bag's flag-generic list does *not* include the internal
  field. A struct with two non-internal fields gets
  `Factory<F1, F2>`, not `Factory<F1, F2, F3>`.
- No setter, remover, overrider, or default helper is emitted
  for it.
- The getter is unconditional — callable on any bag shape,
  because the field is set from the moment `new()` returns.

Internal fields are deliberately incompatible with `optional`,
`default`, `overridable`, `removable`, `setter`, `fallible`,
`async_fn`, and `input`. Using any of them errors at expansion.
The whole point is "locked"; the conflicting attributes would
unlock it.
