### Shared infrastructure

A single `Profile` bag spanning the three test cases:

- `name: String` (required, no transformer) — control field.
- `worker: Option<String>` (storage type) with
  `setter = wrap_some, input = String` — user-facing setter takes
  `String`, the `wrap_some` transformer lifts to `Option<String>`.
- `worker` also has `default = None` so the default helper has to
  bypass the transformer (the `None` literal is already
  `Option<String>`).

The `wrap_some(s: String) -> Option<String>` helper is the
transformer the macro routes the setter call through.
