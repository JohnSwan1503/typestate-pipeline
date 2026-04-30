# Impl-level attribute forwarding

`#[transitions]` decorates a single `impl` block but emits *two* impl
blocks — one for the Resolved-mode carrier and one for the InFlight
carrier. Whatever attributes the user wrote on their source impl have
to end up on *both* generated arms, or the two impl blocks would
diverge and break under any user-level `#[cfg(...)]` gate.

The most useful case is `#[cfg(...)]`: a user can write a single impl
block under a feature gate and have both arms come and go together.
Method-level attribute forwarding (already covered in
`#[transitions]`'s codegen before this regression test was added)
handles `#[allow]` / `#[deny]` / etc. on a single transition;
impl-level forwarding is what makes whole conditionally-compiled
blocks work.

This file pins the impl-level forwarding with a single test that
exercises both arms of a transition emitted under `#[cfg(all())]` —
an always-true cfg that stands in for any real user-level gate.
