# `#[field(input = T)]` — setter input type ≠ stored field type

`#[field(input = T)]` (paired with `setter = my_fn`) lets the setter
accept a different *input* type than the field stores; the
transformer bridges the gap. The canonical use case is an
`Option<T>` field where the user shouldn't have to wrap in `Some(...)`
at every call site — set `input = T` and let a `wrap_some`
transformer lift the value.

When a `default = …` is also declared, the default helper bypasses
the transformer and writes the *field* type directly — the default
expression is evaluated as the storage type, not the input type.

This suite pins three properties:

- The user-facing setter accepts the input type.
- The default helper emits a direct field write (skipping the
  transformer) so a `default = None: Option<String>` works even
  though the setter's input is `String`.
- Leaving the field unset still resolves to the default at
  `finalize()`.
