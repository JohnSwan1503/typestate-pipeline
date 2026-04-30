## Carrier-Side `override_<field>`

**Invariant.** An `overridable` field's `override_<field>(self, val)`
method is emitted on the carrier (Resolved + InFlight) and correctly
keeps the flag at `Yes` while replacing the stored value. The old
value's destructor runs before the new bag is constructed (panic-safe
ordering), and the new value is what `finalize` ultimately yields.

**Failure mode this guards.** The override codegen has two ordering
hazards:

1. **Old value double-drop / leak.** The `MaybeUninit`-mode codegen
   has to read the old value out into a stack temp, store the new
   value, and let the temp drop at scope exit. A wrong ordering can
   leak the old value or double-drop it.
2. **Flag mistransition.** A wrong codegen path could emit
   `override_<field>` as `Yes -> No` (treating it like `drop` + set);
   downstream chains expecting the flag to stay at `Yes` would fail to
   compile.

**Setup.** Set `parallelism = 2` via the regular setter, then call
`override_parallelism(8)`. The override flips no flag — `parallelism`
stays at `Yes(8)`.

The carrier's `deploy` transition is bounded on `parallelism`'s flag
being *`No`* (so the default fires at finalize). To exercise the
override path's stored value we sidestep `deploy` and finalize the bag
directly via `pipeline.0.into_state().finalize()` — the bag finalize
returns the stored value when the flag is `Yes`.

**Assertion.** The finalized dataset's `parallelism == 8` — the
override took effect.

### pipeline_override_replaces_value
