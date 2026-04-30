# Pipeline-arm setter integration tests

`#[factory(pipeline(carrier = MyCarrier))]` instructs the
`TypestateFactory` derive to *also* emit setters / removers /
overriders / default helpers / getters on the user's `Pipeline`-newtype
carrier — so callers can build a bag *through* the carrier in either
`Resolved` or `InFlight` mode without `into_state()` /
`Pipeline::resolved` plumbing.

This suite pins four properties of the generated arm:

- Setters chain in `Resolved` mode and threading through bag finalization
  reaches a follow-on phase via a separate `#[transitions]` impl.
- Setters chain in `InFlight` mode — the bag's fallible-setter `Result`
  folds into the pending future just like any other transition body.
- `drop_<field>` flows through the carrier, flipping a flag `Yes -> No`
  so the field can be re-set.
- `override_<field>` flows through the carrier, replacing a stored
  value while keeping the flag `Yes`.

The four tests share a `DatasetData` bag with one of every relevant
field shape (`required`, `required + removable`, `default + overridable`,
`required + setter + fallible`) so each test exercises real method
generation from the macro.
