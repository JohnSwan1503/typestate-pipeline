## Empty Bag Drop

**Invariant.** A fresh bag (every flag still `No`, every storage
slot still `MaybeUninit::uninit()`) drops cleanly without invoking
`drop` on any storage slot. The generated `Drop` impl reads each
flag's `IS_SET` constant at runtime and skips the slots whose flag
is `No`.

**Failure mode this guards.** If the generated `Drop` impl
unconditionally called `assume_init_drop` on every slot regardless
of flag state, the empty bag would invoke `Counted::drop` on
uninitialized memory — UB.

**Setup.** `DropTrace` (two required `Counted` fields). Construct
a fresh `DropTraceFactory::new()` inside an inner block, let it
drop at end-of-scope.

**Assertion.** `alive() == baseline` — the drop didn't touch the
unset fields.

### empty_bag_dropped_does_not_touch_uninit_fields
