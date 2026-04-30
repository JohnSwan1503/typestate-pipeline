## Internal at `finalize`

**Invariant.** Internal fields are stored as plain `T` (no
`MaybeUninit`, no flag). `finalize()` reads them directly into the
constructed struct without touching the unsafe-mode `MaybeUninit`
machinery the regular fields use.

**Failure mode this guards.** A buggy codegen could route the
internal field through the `MaybeUninit` path, forcing it to
participate in the flag bookkeeping the rest of the bag uses.
That would:

- Add a runtime check that the internal field is "set" (it's
  always set, but the codegen doesn't know that without a special
  case).
- Risk a `MaybeUninit` `assume_init` on a constant value
  (technically fine, but unnecessary unsafe).

The test exercises the simplest case where finalize is called on a
fresh bag without any setter chain having advanced the
non-internal flags — wait, that wouldn't compile because finalize
needs every required flag at `Yes`. Refined: the test sets the
required `parallelism` and lets `verify` default, then finalizes.
The internal field's `namespace` flows through unchanged.

**Setup.** `JobFactory::new("solana".to_owned()).parallelism(1).finalize()`.

**Assertion.** `job.namespace == "solana"` — the internal value is
preserved verbatim.

### finalize_passes_internal_field_through
