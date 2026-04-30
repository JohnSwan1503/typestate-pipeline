## Carrier-Side `drop_<field>`

**Invariant.** A `removable` field's `drop_<field>(self)` method is
emitted on the carrier (with both `Resolved` and `InFlight` arms) and
correctly transitions the field's flag from `Yes` back to `No`,
allowing the field to be re-set afterward without an `override`.

**Failure mode this guards.** Two failure modes:

1. **Drop flag-transition wrong.** If the carrier-arm `drop_<field>`
   emitted a method that left the flag at `Yes`, calling the
   following setter (`network` again) would fail to compile — the
   bag's setter requires the input flag to be `No`.
2. **Drop value-leak.** Internally, `drop_<field>` reads the old value
   into a stack temp before constructing the new bag (so a panicking
   destructor unwinds with the new bag in scope and its own destructor
   reclaims the rest). If that ordering broke, the leak would show up
   under `Counted`-style bookkeeping. (Bookkeeping isn't wired into
   *this* test — that's `safety::factory_no_leak`'s job; here we only
   check the value side.)

**Setup.** Set `network = "eth"`, drop it, set `network = "op"`. The
chain has to typecheck, which is itself the proof that the flag flip
worked.

**Assertion.** After `deploy`, the deployed dataset's
`network == "op"` — the second value, not the first.

### pipeline_drop_field_transitions_yes_to_no
