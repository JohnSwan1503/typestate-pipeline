### Factory in pipeline: bag built outside the carrier

The earlier pipeline-arm section showed the carrier *as* the
fluent surface for setting fields — convenient when "build the
bag" and "advance the carrier" are the same chain. The
alternative is when they aren't: the bag is assembled
elsewhere — read from configuration, decoded from a request,
constructed by a dedicated builder in another module — and
arrives at the carrier already populated.

In that case the bag is just a normal struct, the carrier is
a normal phase machine, and the only meeting point is a
`#[transitions]` body that takes the assembled struct as part
of one phase's state and uses it to compute the next phase.
The bag's `finalize()` produces the struct outside the carrier;
the transition reads it inside.

The two scenarios are complementary:

- **Pipeline arm** is the right shape when the carrier owns the
  build process. Setters live on the carrier; the chain stays
  flat.
- **Factory in pipeline** is the right shape when the bag's
  origin is outside the carrier's responsibility. The carrier
  receives a finished value and advances from there.

The example below demonstrates the latter. Compare it against
the pipeline-arm example in the factory section — same domain,
different boundary.
