## Explicitly-Set Optionals

**Invariant.** A `Profile` bag with every flag at `Yes` (including
the optional `age`) implements `ProfileFactoryReady`. The trait
applies to *both* `(Yes, Yes, No)` and `(Yes, Yes, Yes)` shapes —
matching `finalize`'s bounds.

**Failure mode this guards.** Symmetric to the previous test: a
regression that pinned the optional flag to `No` in the trait's
auto-impl (instead of accepting either) would refuse to compile
this case.

**Setup.** Bag with all three fields set explicitly, including the
optional via `with_age(42)`.

**Assertion.**

- `generic_finalize` compiles for the fully-set bag.
- `profile.age == 42` — the explicit value, not the default.

### ready_trait_works_when_optional_set_too
