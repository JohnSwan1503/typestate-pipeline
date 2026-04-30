## Arbitrary Order Setters

**Invariant.** Setters can be called in any order. The bag's flag
generic list is order-independent — `Factory<Yes, Yes, Yes>` is the
same type regardless of which setter flipped which flag first.

**Failure mode this guards.** If the safe-mode codegen emitted setters
that pinned other fields' flags to specific values (rather than leaving
them free), a different call order would not typecheck.

**Setup.** Same `UserBuilder` as `build_in_order`. Setters called
`age` → `email` → `name` (reverse-ish of declared order).

**Assertion.** Final struct still has the right values for `name` and
`age`. The chain typechecking at all is most of the proof.

### build_in_arbitrary_order
