## Carrier Non-Internal Getter

**Invariant.** A non-internal field's carrier-arm getter
(`carrier.parallelism()`) is only callable once that field's flag
has advanced to `Yes`. The internal getter on the same carrier
remains callable regardless.

**Failure mode this guards.** Two failure shapes:

1. **Non-internal getter ungated.** A buggy codegen could emit the
   non-internal getter on an all-shapes impl, letting users read
   the slot before any value has been written — `MaybeUninit`
   undefined behavior in unsafe-mode, garbage values in safe-mode.
2. **Internal getter spuriously gated.** A buggy codegen could
   accidentally restrict the internal getter to the same flag
   bound it gave the non-internal getter, breaking the
   "constant-from-construction" semantics.

**Setup.** Carrier opened with `namespace = "eth"` (internal),
then `.parallelism(16)` advances that flag. Both getters are
called on the configured carrier.

**Assertion.** `*configured.parallelism() == 16` and
`configured.namespace() == "eth"`. The compile is the witness for
the gating; the value checks confirm correct storage.

### carrier_non_internal_getter_gates_on_yes_flag
