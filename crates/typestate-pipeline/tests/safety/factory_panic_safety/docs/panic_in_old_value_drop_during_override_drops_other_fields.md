## Panic In Overridden Value Drop

**Invariant.** When `override_<field>(self, val)` panics inside the
*old* value's `T::drop`, the bag's other fields must drop. This is the
override-specific shape of the panic-safety guarantee.

**Failure mode this guards.** The naive override codegen called
`assume_init_drop` on the old value before constructing the new bag:

```text
let new_bag_marker = ...;
self.a.assume_init_drop();  // panic here unwinds with NO new bag in scope
let new_bag = NewBag {
    a: MaybeUninit::new(new_value),
    b: ptr::read(&self.b),  // never reached
    ...
};
```

A panic in the old `a`'s `Drop` aborted before the new bag was
constructed. `b` and `c` remained inside the original bag's
`ManuallyDrop`-wrapped `self` and leaked.

The fix moves the old-value drop to *after* the new bag is built —
read the old value into a stack temp, build the new bag (whose
panic-safe `Drop` covers `b` and `c`), then let the temp auto-drop at
end-of-scope:

```text
let __tsh_old = self.a.assume_init_read();
let __tsh_new_bag = NewBag { a: ..., b: ptr::read(&self.b), c: ptr::read(&self.c) };
// __tsh_old auto-drops here; if it panics, __tsh_new_bag is in scope
// and unwinds via the bag's panic-safe Drop, releasing b and c.
__tsh_new_bag
```

**Setup.** A bag with `a: PanickyDrop` (overridable), and required
`b: Counted`, `c: Counted`. After construction `alive() == 2`.
`override_a(PanickyDrop)` is called inside `catch_unwind` — the *old*
`a`'s `Drop` panics.

The new value is also a `PanickyDrop`, but the one-shot fuse is spent
by the old value's panic, so its `Drop` is a no-op during unwind.

**Assertion.** After `catch_unwind` returns `Err`, `alive() == 0`:
both `b` and `c` were dropped by the new bag's panic-safe `Drop`
during unwind.

### panic_in_old_value_drop_during_override_drops_other_fields
