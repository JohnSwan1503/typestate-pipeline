## Empty Bag Drop

**Invariant.** A fresh bag — every flag still `No` — drops cleanly:
no field destructors run, no leaks. In safe mode, every storage slot
is `()`, so the auto-derived `Drop` is effectively a no-op.

**Failure mode this guards.** If safe mode emitted a manual `Drop`
impl (which it shouldn't — that's the whole point), it could
accidentally try to destruct the `()` slots as if they were `T`.
Crash. The test exercises the simplest case where every storage slot
is the unit type.

**Setup.** `DropTrace` with two `Counted` fields, neither set. Bag
goes out of scope inside an inner block.

**Assertion.** `alive() == baseline` after the bag drops — no
`Counted::drop` call ran (because no `Counted::new` ran either).

### empty_bag_dropped_does_not_touch_unset_fields
