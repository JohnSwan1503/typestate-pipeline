## Custom Bag Name

**Invariant.** `#[factory(name = MyManifestBuilder)]` overrides the
default `<Original>Factory` bag type name. The generated factory
exists under the user's chosen name; the default name is *not*
also emitted (no shadow type).

**Failure mode this guards.** A buggy codegen could either:

- Emit both the user's name *and* the default-derived name —
  doubling compile time and conflicting if the user later defines
  a struct named `<Original>Factory`.
- Emit only the default name — the user's `name = ...` arg is
  silently ignored; calling `MyManifestBuilder::new()` fails to
  resolve.

**Setup.** `ManifestData` struct with
`#[factory(name = MyManifestBuilder)]`. Only `MyManifestBuilder`
is referenced in the test.

**Assertion.** `MyManifestBuilder::new().title(...).finalize()`
typechecks and yields a `ManifestData` with the right value.

### custom_bag_name
