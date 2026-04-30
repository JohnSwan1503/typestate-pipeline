## Fallible Overrider Failure

**Invariant.** When a `fallible` `override_<field>` rejects its input,
the bag must drop *both* the previously-stored value of the overridden
field *and* every other set field. Override semantics: if the new value
is rejected, the bag is left in its prior state — but in this test we
verify the stronger property that the failing path runs every relevant
destructor.

**Failure mode this guards.** The pre-fix codegen ran the transformer
inside the `ManuallyDrop` scope *and* `assume_init_drop`-ed the OLD
value of the overridden field before the transformer was called. So
when the transformer rejected, two things went wrong at once:

1. The OLD value of `main` had already been destroyed with no
   replacement to take its slot — a phantom drop.
2. `other` was still sitting in `MaybeUninit` storage suppressed by
   `ManuallyDrop`, leaking on the `Err` branch.

This is the worst-case manifestation of the leak bug: a fallible
overrider failure both destroyed and leaked at the same time.

**Setup.**

- `OverridableBag` — two required `Counted` fields. `main` is
  `overridable` with a fallible transformer `sync_pass_or_reject` that
  rejects any value labelled `"fail"`.
- The bag is built with `other` and an initial `main = "m_old"` —
  `alive() == baseline + 2`.
- `override_main(Counted::new("fail"))` is called; the transformer
  rejects after dropping its input.

**Assertion.** After the failing override returns `Err`, `alive()` is
back to `baseline` — both `other` and `m_old` were dropped by the bag's
panic-safe `Drop`. With the buggy codegen, `m_old` would be gone and
`other` would still be alive.

### fallible_overrider_failure_drops_other_set_fields
