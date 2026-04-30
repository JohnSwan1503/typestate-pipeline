## Flag List Excludes Internal

**Invariant.** A bag with one internal field and two non-internal
fields has the type signature `Factory<F1, F2>` — only the
non-internal flags are exposed as generic parameters.

**Failure mode this guards.** A buggy codegen could include the
internal field in the flag-generic list, producing
`Factory<InternalFlag, F1, F2>`. That would:

- Force users to spell out three flags everywhere even though the
  internal one is constant.
- Break ergonomics around the `<Bag>Ready` trait (the trait would
  need a bound on the internal flag too).

The test pins the absence by *type-annotating* the bag as
`JobFactory<No, No>` — only two type parameters. If the codegen
included the internal in the flag list, the annotation would
require three params and the test would fail to compile.

**Setup.** `let bag: JobFactory<No, No> = JobFactory::new("eth".to_owned());`
— the type annotation is the test.

**Assertion.** The annotation typechecks. Compile-time witness only.

### internal_field_dropped_from_flag_generic_list
