## Default Helper

**Invariant.** A field declared with `default = expr` gets a
`<field>_default()` helper that writes the default expression's value
to storage and flips the flag from `No` to `Yes`. Subsequent
`finalize()` reads the explicit value, not the default branch.

**Failure mode this guards.** If the safe-mode `<field>_default()`
helper instead left the flag at `No` and relied on `finalize` to
evaluate the default, the test wouldn't observe a difference (both
paths produce the same value in this case). But if the helper *also*
short-circuited finalize's bound, calling `<field>_default()` and then
calling another `with_<field>(...)` would be ambiguous; the test pins
that the helper truly transitions the flag.

**Setup.** `UserBuilder` with `age` defaulting to 18. `age_default()`
is called instead of `with_age(...)`.

**Assertion.** `user.age == 18`.

### default_helper_fills_in_optional
