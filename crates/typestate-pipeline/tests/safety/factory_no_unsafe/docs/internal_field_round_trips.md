## Internal Round-Trip

**Invariant.** A `#[field(internal)]` field is positional on `new(...)`,
has an unconditional getter, and round-trips through `finalize()`
intact. The field doesn't appear in the bag's flag generic list at
all — internal fields aren't part of the typestate.

**Failure mode this guards.** Internal handling is a separate codegen
path from the regular flag-tracked fields. A safe-mode regression
that "lost" the internal field's value somewhere — e.g. by storing it
under a `()` slot for a non-existent flag — would surface as either a
finalize-time mismatch or a getter returning the wrong value.

**Setup.** `WithInternal` with `namespace` (internal) and `name`
(required). `new("ns".to_owned())` constructs the bag with `namespace`
set; the internal getter is called immediately to confirm it's
already accessible (no flag bound). Then `name(...)` and `finalize()`.

**Assertion.**

- `bag.namespace() == "ns"` (getter on the empty bag).
- After finalize: both `user.namespace == "ns"` and
  `user.name == "svc"`.

### internal_field_round_trips
