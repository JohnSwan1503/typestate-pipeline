## Positional Constructor

**Invariant.** `JobFactory::new("eth".to_owned())` takes the
internal `namespace` field as a positional argument. The result is
a fresh bag with the internal field already populated and the
non-internal flags still at `No`. Setter chain proceeds normally
on the user-facing fields.

**Failure mode this guards.** A buggy codegen could:

- Make `new` take zero arguments and require a separate
  `.namespace(...)` call — but no setter exists for internal
  fields, so this would be a dead-end signature.
- Take the internal field but store it under a wrong slot,
  surfacing as a value mismatch at finalize.

**Setup.** `JobFactory::new("eth".to_owned()).parallelism(4).with_verify(true).finalize()`.

**Assertion.** Finalized job has `namespace == "eth"`,
`parallelism == 4`, `verify == true`.

### constructor_takes_internal_field_as_argument
