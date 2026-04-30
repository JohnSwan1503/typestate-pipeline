## Panic In Removed Value Drop

**Invariant.** Same shape as the override case: when `drop_<field>(self)`
panics inside the removed value's `T::drop`, the bag's other fields
must drop on unwind.

**Failure mode this guards.** Identical structure to the override
failure mode — the pre-fix codegen ran the removed value's destructor
before constructing the new bag (whose flag for the removed field is
`No`, no value stored). A panic during the removed value's `Drop`
aborted before the new bag was constructed; the original bag's other
fields leaked inside `ManuallyDrop`.

The fix is the same as for override: read the old value into a stack
temp first, build the new bag, then let the temp auto-drop at
end-of-scope.

**Setup.** A bag with `a: PanickyDrop` (`removable`), required
`b: Counted`, `c: Counted`. `bag.drop_a()` is called inside
`catch_unwind` — the removed `a`'s `Drop` panics.

Unlike the override test, no fresh `PanickyDrop` is constructed (drop
removes a value, doesn't replace it). The fuse only fires once on the
old value.

**Assertion.** After `catch_unwind` returns `Err`, `alive() == 0`:
the new bag (without `a`) was constructed and its panic-safe `Drop`
released `b` and `c` during unwind.

### panic_in_old_value_drop_during_remove_drops_other_fields
