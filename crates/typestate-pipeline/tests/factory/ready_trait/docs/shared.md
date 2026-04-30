### Shared infrastructure

- `Profile` — three-field bag with two `required` fields and one
  `default = 18` field. The default field is what makes the trait
  impl bound interesting: it auto-impls on `(Yes, Yes, _)` (any
  state for `age`'s flag).
- `generic_finalize<B: ProfileFactoryReady>(bag) -> Profile` — a
  helper generic over the auto-emitted trait. Its where-clause is
  the entire test surface: if the trait isn't impl'd correctly for
  the bag, the call doesn't compile.
