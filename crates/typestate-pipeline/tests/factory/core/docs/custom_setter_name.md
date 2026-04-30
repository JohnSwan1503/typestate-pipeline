## Custom Setter Name

**Invariant.** `#[field(required, name = shout_name)]` renames the
setter from `name(...)` to `shout_name(...)`. The getter and
default helper retain the field name (`name()` / `name_default()`)
— only the setter is renamed.

**Failure mode this guards.** A buggy codegen could:

- Apply the rename to every method (getter + helper too) — would
  break `<bag>.<field>()` getter calls.
- Ignore the rename and emit `name(...)` — the test's
  `shout_name(...)` would fail to resolve.

**Setup.** `LoudUser` struct with `#[field(required, name = shout_name)]`.

**Assertion.** `LoudUserFactory::new().shout_name("ALICE".to_owned()).finalize()`
typechecks and yields a `LoudUser` with `name == "ALICE"`.

### custom_setter_name
