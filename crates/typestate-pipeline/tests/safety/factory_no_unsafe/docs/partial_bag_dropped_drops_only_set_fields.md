## Partial Bag Drop

**Invariant.** A bag with some fields set and some unset, when
dropped, runs destructors only for the set fields. This is what
auto-Drop on `<Flag as Storage<T>>::Out` gives us "for free": `Yes`
slots are `T` (auto-drop calls `T::drop`), `No` slots are `()`
(auto-drop is a no-op).

**Failure mode this guards.** If the safe-mode codegen got the
sister-struct shape wrong (e.g. pinned both fields to `T` even when
one flag is `No`), auto-Drop would call destructors on `()` slots —
either UB or a wrong count. The test pins the count to exactly one.

**Setup.** `DropTrace` with `primary` set, `secondary` unset. Counter
checked at `baseline + 1` between set and drop.

**Assertion.** After the bag drops, `alive() == baseline` — exactly
one destructor ran.

### partial_bag_dropped_drops_only_set_fields
