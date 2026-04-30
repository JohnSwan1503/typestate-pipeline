## Drop-Then-Reset

**Invariant.** `set -> drop_<field> -> set` is a valid lifecycle. The
first set's value is dropped exactly once (during `drop_<field>`);
the second set's value is dropped exactly once (when the bag itself
drops). No interaction between the two values.

**Failure mode this guards.** The setter and the remover share
codegen patterns. A regression that "remembered" the old slot's
contents on the new bag (e.g. by carrying a residual through the
sister-struct shape) could double-drop on the second set or on
final bag drop.

**Setup.** `Removable` bag with `primary("p1")` and `other("o")`,
then `drop_primary()` releasing `"p1"`, then `primary(Counted::new("p2"))`
storing the second value. Final state: two `Counted`s alive
(`other` and `p2`).

**Assertion.** `alive() == baseline + 2` mid-way; `alive() ==
baseline` after the bag drops. Both `p1` and `p2` were dropped exactly
once each.

### drop_field_then_reset_doesnt_double_drop
