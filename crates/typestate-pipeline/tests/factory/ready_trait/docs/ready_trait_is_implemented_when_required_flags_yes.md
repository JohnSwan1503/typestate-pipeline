## Required Fields Set

**Invariant.** A `Profile` bag with both required fields set
(`name`, `handle`) but the optional `age` left unset still
implements `ProfileFactoryReady`. The trait's auto-impl bounds
match `finalize()`'s — required fields must be `Satisfied`,
optional-with-default fields can be either.

**Failure mode this guards.** A regression that incorrectly
generated the trait's auto-impl bounds (e.g. requiring
`Satisfied` on the optional `age` flag too) would refuse to
implement the trait for the partial bag. The `generic_finalize`
call would then be a compile error.

**Setup.** Build a bag with `name` and `handle` set, `age` left
at its `No` flag (the default = 18 will fire at finalize).

**Assertion.**

- The call to `generic_finalize(bag)` compiles — proof the trait
  is impl'd for the partial bag.
- The resulting profile has `name == "alice"`, `handle == "@alice"`,
  `age == 18` (the declared default).

### ready_trait_is_implemented_when_required_flags_yes
