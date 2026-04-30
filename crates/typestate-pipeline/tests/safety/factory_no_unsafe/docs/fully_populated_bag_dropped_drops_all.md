## Fully-Set Bag Drop

**Invariant.** A bag with every required field set, dropped without
finalizing, releases all stored values via auto-Drop on the
sister-struct's `T` slots.

**Failure mode this guards.** This is the symmetric case to
[`partial_bag_dropped_drops_only_set_fields`](../partial_bag_dropped_drops_only_set_fields/index.html).
Together they pin: zero `Yes`-flagged fields → zero destructors,
N `Yes`-flagged fields → N destructors.

**Setup.** `DropTrace` with both `primary` and `secondary` set.

**Assertion.** After the bag drops, `alive() == baseline` — both
`Counted::drop`s ran.

### fully_populated_bag_dropped_drops_all
