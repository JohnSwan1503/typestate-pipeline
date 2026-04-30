## Declared-Order Setters

**Invariant.** A setter chain that walks every required field in
declared order finalizes to a struct whose values match what was
passed in.

**Failure mode this guards.** A regression in the safe-mode codegen
that wrote field values into the wrong sister-struct slot would
surface here as a value mismatch.

**Setup.** A `UserBuilder` with `name`, `email` (both required) and
`age` (default 18). Setters called in source order, with `age`
explicitly overridden to 30.

**Assertion.** All three field values match the inputs.

### build_in_order
