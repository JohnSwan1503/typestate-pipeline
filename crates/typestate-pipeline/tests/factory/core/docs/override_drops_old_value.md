## Override Drops Old Value

**Invariant.** `override_<field>(self, val)` on a `Yes`-flagged bag
runs the prior stored value's `Drop`, then stores the new value,
keeping the flag at `Yes`.

**Failure mode this guards.** The override codegen has to read the
old value into a stack temp before the new bag is constructed
(panic-safe ordering — see
[`tests::safety::factory_panic_safety::panic_in_old_value_drop_during_override_drops_other_fields`](../../safety/factory_panic_safety/panic_in_old_value_drop_during_override_drops_other_fields/index.html)).
That ordering also has to result in a single drop. Two regressions:

- Old value leaked (no drop run).
- Old value double-dropped (the temp dropped *and* the new bag's
  `Drop` runs on the slot).

**Setup.** `Overridable` bag with `payload(Counted::new("first"))`,
then `override_payload(Counted::new("second"))`.

**Assertion.** Mid-scope, `alive() == baseline + 1` — the first
value dropped, the second is alive. After the bag drops,
`alive() == baseline` — the second value dropped.

### override_drops_old_value
