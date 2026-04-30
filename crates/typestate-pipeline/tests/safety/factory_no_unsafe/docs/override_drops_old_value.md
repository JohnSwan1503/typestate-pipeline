## Override Drops Old Value

**Invariant.** `override_<field>(self, val)` on a `Yes`-flagged bag
drops the prior stored value and stores the new one, with the flag
remaining at `Yes`.

**Failure mode this guards.** Three failure modes:

1. **Old value leaked.** If the override rebuilt the bag with the new
   value but didn't drop the old, the count stays inflated.
2. **Old value double-dropped.** If both the override and the bag's
   auto-Drop drop the old slot, count goes negative.
3. **New value not stored.** If a wrong shape pulled `()` into the
   `Yes` slot, the replacement is lost (and the field's value at
   finalize would be wrong).

**Setup.** `Overridable` bag with `payload(Counted::new("first"))`,
then `override_payload(Counted::new("second"))`.

**Assertion.** Mid-way, `alive() == baseline + 1` — the first value
dropped, the second alive. After the bag itself drops, `alive() ==
baseline` — the second value dropped.

### override_drops_old_value
