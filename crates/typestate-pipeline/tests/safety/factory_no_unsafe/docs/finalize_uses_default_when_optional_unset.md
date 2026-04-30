## Default Eval at `finalize`

**Invariant.** A bag whose optional-with-default field's flag is still
`No` at finalize time produces a struct whose field value comes from
the default expression, not from any default of the field type.

**Failure mode this guards.** In safe mode, the dispatch is via
`Storage::finalize_or` — a trait method picked at monomorphization
based on the flag type. If the trait impl for `No` evaluated the
storage `()` instead of calling the default thunk, the field would
hold whatever `()` produced (zero-init for primitives, garbage for
non-trivial types), not the declared default.

**Setup.** Same `UserBuilder`. `name` and `email` are set; `age`
flag stays `No`.

**Assertion.** `user.age == 18` — the declared default value, not
zero.

### finalize_uses_default_when_optional_unset
