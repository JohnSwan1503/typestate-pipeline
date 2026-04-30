## Default Helper

**Invariant.** A field declared with `default = expr` gets a
`<field>_default()` helper that writes the default expression's
value to storage and flips the flag from `No` to `Yes`.

**Failure mode this guards.** A buggy codegen could omit the helper
entirely (forcing users to either set the field explicitly or let
finalize evaluate the default), or could emit the helper without
flipping the flag (leaving the bag in an inconsistent state where
the value is set but the flag thinks it isn't).

**Setup.** `UserBuilder` with `age` defaulting to 18. Chain:
`name(...).email(...).age_default().finalize()`.

**Assertion.** `user.age == 18` — the default was written.

### default_helper_fills_in_optional
