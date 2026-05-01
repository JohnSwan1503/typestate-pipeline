## Putting it together: macro combinations

The factory and the carrier macros stand on their own, but
real codebases compose them. There are two interlock points
worth distinguishing, because they put the boundary between
"build the bag" and "advance the carrier" in different places.

The first is **carrier-as-builder**: the bag's setters are
emitted directly on the user's carrier through
`#[factory(pipeline(carrier = …))]`, so the carrier *is* the
fluent surface for both filling fields and advancing phases.
The chain reads as one expression — setter, setter, transition,
setter, transition, finalize — and never leaves the carrier.
That shape is covered in
[Pipeline arm](#pipeline-arm-setters-on-the-carrier).

The second is **bag-handed-in**: the bag is assembled
elsewhere — read from configuration, decoded from a request,
constructed by a dedicated builder in another module — and a
`#[transition]` body takes the finished value as input to the
next phase. The bag and the carrier never share a chain; they
meet at one transition's argument list. That's the shape
covered below.

The two are complementary, not alternatives. A pipeline can use
one in some phases and the other in others, depending on which
side of the boundary owns the bag's construction.
