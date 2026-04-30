## Arbitrary Order Setters

**Invariant.** Setter ordering doesn't matter — the typestate
transitions independently per field. `Factory<Yes, Yes, Yes>` is
the same type regardless of which setter flipped which flag first.

**Failure mode this guards.** A buggy codegen could pin other
fields' flags to `No` in a setter's input bag bound (rather than
leaving them free generic). With that pin, calling setters in a
different order would fail to typecheck.

**Setup.** Same `UserBuilder` as `build_in_order`. Setters called
`age` → `email` → `name` (reverse-ish of declared order).

**Assertion.** Final struct still has the right values for every
field. Compile-witness on the chain typing.

### build_in_arbitrary_order
