# `#[derive(TypestateFactory)]`

Generates `<Name>Factory<F1, F2, …>` with one flag generic per field.
Setters consume `self` and transition the relevant flag from `No` to
`Yes`. `finalize()` is callable only when every required flag is `Yes`.

```rust
use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct User {
    #[field(required)]
    name: String,
    #[field(required)]
    email: String,
    #[field(default = 18)]
    age: u32,
}

let user = UserFactory::new()
    .name("Alice".into())
    .email("alice@example.com".into())
    .with_age(30)        // optional fields → `with_<field>`
    .finalize();
```

Selected features:

- `setter = my_fn` + `fallible` / `async_fn` — call a transformer inside
  the setter (sync, fallible, async, or async-fallible).
- `default` / `default = expr` — declare a default; emits a
  `<field>_default()` helper. Optional-with-default fields can finalize in
  either flag state.
- `removable` — emit `drop_<field>(self)` reverting the flag to `No`.
- `overridable` — emit `override_<field>(self, val)` on `Yes`-flagged bags.
- `internal` — set positionally at `new(…)`, locked from then on.
- `pipeline(carrier = …)` — also emit Resolved + InFlight method pairs
  on the carrier.
- `finalize_async(via = …, into = …)` — async finalize hook.

See [`TypestateFactory`](crate::TypestateFactory) for the full attribute
reference.
