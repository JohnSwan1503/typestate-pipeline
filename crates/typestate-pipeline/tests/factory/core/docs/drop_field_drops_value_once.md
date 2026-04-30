## Drop Field

**Invariant.** `drop_<field>(self)` on a `Yes`-flagged bag returns
a new bag with that field's flag at `No`. The previously-stored
value's `Drop` runs exactly once during the transition; other
fields are preserved unchanged.

**Failure mode this guards.** Two regressions:

1. **Missing drop.** A buggy codegen could rebuild the new bag
   with the slot at `MaybeUninit::uninit()` without first running
   `assume_init_drop` on the old value — leak.
2. **Double drop.** A buggy codegen could run the value's
   destructor *and* then have the new bag's `Drop` impl also
   destruct the slot — UB.

**Setup.** `Removable` bag with `primary` and `other` both set.
Counter checked at `baseline + 2`. Call `drop_primary()`. Counter
should be at `baseline + 1` between drop_primary and bag drop.

**Assertion.**

- After `drop_primary()`: `alive() == baseline + 1` (primary's
  destructor ran).
- After the bag drops: `alive() == baseline` (other's destructor
  ran).

### drop_field_drops_value_once
