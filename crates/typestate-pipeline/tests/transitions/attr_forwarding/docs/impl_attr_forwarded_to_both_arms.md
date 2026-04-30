## Attribute Forwarding

**Invariant.** Attributes on the source `impl` block decorated with
`#[transitions]` propagate to *both* generated arms. A user `#[cfg(...)]`
gate that hides the source impl must also hide both generated impls,
not just one.

**Failure mode this guards.** A buggy codegen could:

- Forward the attribute only to the Resolved arm, leaving the InFlight
  arm always emitted. The InFlight arm references types the cfg may
  have deleted — compile error in cfg-off mode.
- Drop the attribute entirely. Both arms are always emitted, leaking
  the gated impl across cfg flips.

The test uses `#[cfg(all())]` (always true) as a deliberate stand-in:
the cfg itself doesn't matter, but the *forwarding* does. If forwarding
were broken, calling either arm in this test would either fail to
compile (if one was missing) or accidentally compile when it shouldn't
(if the cfg gate was dropped).

`#[allow(clippy::non_minimal_cfg)]` silences clippy's complaint about
`cfg(all())` being redundant — it *is* redundant for production code,
but here it's intentional probe machinery.

**Setup.** A trivial `Started -> Finished` `#[transition]` decorated
with `#[cfg(all())] #[transitions]`. The body is a no-op; what we're
testing is the impl block survived in both arms.

**Assertion.**

- `Author(Pipeline::resolved(...)).finish()` returns
  `Author<Finished, Resolved>` — proves the Resolved arm exists.
- `Author(Pipeline::in_flight(...)).finish().await.unwrap()` also
  returns `Author<Finished, Resolved>` — proves the InFlight arm
  exists and folds back via `IntoFuture`.

If the cfg attr were forwarded to only one arm, one of these calls
would be a compile error.

### impl_attr_forwarded_to_both_arms
