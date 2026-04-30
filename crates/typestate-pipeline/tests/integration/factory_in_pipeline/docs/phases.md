### Phase-state types

The three phases the carrier walks through. `Drafting` is the
unusual one — it carries a fully-set `UserFactory<Yes, Yes, Yes>`
bag rather than a finalized struct, demonstrating that a bag is a
valid phase-state shape. `Submitted(User)` and `Confirmed { user,
confirmation_id }` carry the finalized values produced by the
transitions.
