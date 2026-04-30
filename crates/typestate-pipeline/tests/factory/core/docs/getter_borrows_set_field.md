## Getter on `Yes` Flag

**Invariant.** Each field's `bag.<field>()` getter is callable on
bags whose flag for that field is `Yes`. The getter borrows the
stored value out of the `MaybeUninit` slot via
`assume_init_ref()` — sound because the impl is bounded on the
flag being `Yes` (the type-level witness that the field was
written by the corresponding setter).

**Failure mode this guards.** Two regressions:

1. **Getter ungated.** A buggy codegen could emit the getter on an
   all-shapes impl, so calling it on an unset bag would
   `assume_init_ref` an uninitialized slot — UB.
2. **Getter on the wrong slot.** Indexing math errors would surface
   as a value mismatch (e.g. `bag.name()` returning the email).

**Setup.** `UserBuilder` with `name` and `email` set, `age` left
unset. Both getters are called on the partial bag, then `age` is
defaulted and the bag finalized.

**Assertion.** `bag.name() == "Dave"`, `bag.email() == "dave@example.com"`,
finalized struct has `name == "Dave"`.

### getter_borrows_set_field
