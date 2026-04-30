## Drop-Then-Reset

**Invariant.** A `set -> drop_<field> -> set` lifecycle drops the
first set's value exactly once (during `drop_<field>`) and the
second set's value exactly once (when the bag itself drops). No
interaction between the two values.

**Failure mode this guards.** The setter and remover share codegen
patterns. A regression that "remembered" the old slot's contents
on the new bag (e.g. by reading from a stale flag table) could
double-drop the first value when the bag finally drops, or
double-drop the second value if the new bag's `Drop` thinks
there's still a value at the slot.

**Setup.** `Removable` bag with `primary("p1")` set, `other("o")`
set, `drop_primary()` removes `p1`, `primary(Counted::new("p2"))`
re-sets to `p2`. Final state: two `Counted`s alive (`other` and
`p2`).

**Assertion.** Mid-scope, `alive() == baseline + 2`. After bag
drop, `alive() == baseline`. Each `Counted` was dropped exactly
once.

### drop_field_then_reset_doesnt_double_drop
