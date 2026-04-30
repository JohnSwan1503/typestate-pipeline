## Drop Field

**Invariant.** `drop_<field>(self)` on a `Yes`-flagged bag returns a
new bag with that field's flag at `No` and the storage slot at `()`.
The previously-stored value's `Drop` runs exactly once during the
transition; other fields are preserved.

**Failure mode this guards.** Two regressions:

1. **Forgot to drop.** If the safe-mode `drop_<field>` simply rebuilt
   the bag with a `()` slot but didn't actually drop the prior `T`,
   the `T` would leak (its destructor never running). The bookkeeping
   counter would stay too high.
2. **Double-drop.** If the codegen called `T::drop` *and* let the new
   bag's auto-Drop also fire on the (now invalid) slot, the count would
   go negative or trip UB.

**Setup.** `Removable` bag with `primary` and `other` both set.
Snapshot `alive() == baseline + 2`. Call `drop_primary()`.

**Assertion.**

- Just after `drop_primary()`, `alive() == baseline + 1` (primary's
  destructor ran).
- After the bag itself drops, `alive() == baseline` (other's
  destructor ran).

### drop_field_drops_value_once
