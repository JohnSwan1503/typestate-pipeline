## Declared-Order Setters

**Invariant.** A setter chain that walks every required field in
declared order finalizes to a struct whose values match what was
passed in.

**Failure mode this guards.** A regression in field-storage indexing
would surface as a value mismatch. The `MaybeUninit`-mode codegen
indexes each field's slot via `ptr::read` against `self`'s
`MaybeUninit` array; getting that indexing wrong would write
`Alice` into `email`'s slot or vice-versa.

**Setup.** `UserBuilder` with `name`, `email` (both required) and
`age` (default 18). Setter chain:
`name("Alice").email("alice@example.com").with_age(30)`.

**Assertion.** All three fields hold the inputs verbatim.

### build_in_order
