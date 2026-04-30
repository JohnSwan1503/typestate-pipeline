## `finalize` No Double-Drop

**Invariant.** `finalize()` consumes the bag and produces the
original struct. Each field's value is moved out of the
`MaybeUninit` slot into the resulting struct exactly once. The
bag's `Drop` impl does NOT run a second time on the moved-out
values.

**Failure mode this guards.** The unsafe-mode `finalize()` wraps
`self` in `ManuallyDrop` before reading the fields out via
`ptr::read`. Without that `ManuallyDrop`, after `finalize` returned
the new struct, `self`'s normal `Drop` would fire and `ptr::read`
the now-moved-out slots a second time — double drop, UB. This
test pins the `ManuallyDrop` ordering by counting alive `Counted`s
before and after the bag's lifetime ends.

**Setup.** `DropTrace` fully populated. Call `finalize()` inside an
inner block; hold the resulting `DropTrace` in a local until
end-of-scope.

**Assertion.** Mid-scope, `alive() == baseline + 2` (the values
are alive, owned by the result). After end-of-scope,
`alive() == baseline` (each value dropped exactly once via the
result's auto-Drop). No double-drop, no leak.

### finalize_does_not_double_drop
