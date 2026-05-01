## Fully-Set Bag Drop

**Invariant.** A bag with every required field set, dropped without
finalizing, releases all stored values via the generated `Drop`
impl.

**Failure mode this guards.** Symmetric to
[`partial_bag_dropped_drops_only_set_fields`](#partial_bag_dropped_drops_only_set_fields).
Together they pin: zero `Yes`-flagged fields → zero destructors,
N `Yes`-flagged fields → N destructors. The fully-set case is the
control: if the loop terminates early or skips the last field, the
counter doesn't return to baseline.

**Setup.** `DropTrace` with `primary` and `secondary` both set.
Counter checked at `baseline + 2` before drop.

**Assertion.** After the bag drops, `alive() == baseline`.

### fully_populated_bag_dropped_drops_all
