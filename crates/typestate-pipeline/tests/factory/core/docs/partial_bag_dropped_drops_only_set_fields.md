## Partial Bag Drop

**Invariant.** A bag with some fields set and some unset drops the
set ones (calling each value's `T::drop`) and skips the unset ones
(leaving their `MaybeUninit::uninit()` slots untouched).

**Failure mode this guards.** Same shape as
[`empty_bag_dropped_does_not_touch_uninit_fields`](#empty_bag_dropped_does_not_touch_uninit_fields)
but exercises both branches of the `IS_SET`-guarded loop. If the
flag-check were inverted, the test would either leak (set fields
not dropped) or UB (unset fields dropped).

**Setup.** `DropTrace` with `primary(Counted::new("p"))` set, no
`secondary`. Counter checked at `baseline + 1` between set and drop.

**Assertion.** After the bag drops, `alive() == baseline`. Net
effect: exactly one `Counted::drop` call.

### partial_bag_dropped_drops_only_set_fields
