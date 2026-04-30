## Trait/Inherent Parity

**Invariant.** Calling `finalize()` via the trait produces
the same value as calling the inherent `finalize()` on an
equivalent bag. The trait method is purely a delegation shim — it
must not diverge from the inherent body.

**Failure mode this guards.** The trait's auto-impl emits its own
body, which has to either delegate to the inherent or duplicate
its logic. A regression that duplicated the body but missed a
piece (e.g. forgot the optional-with-default branch) would
silently produce different values via the two paths.

The test exercises exactly this: build two equivalent bags, run
one through the trait method and the other through the inherent
`finalize()`, assert every field matches.

**Setup.** Two equivalent `Profile` bags built with the same
inputs. One goes through `ProfileFactoryReady::finalize(bag)`,
the other through `bag.finalize()`.

**Assertion.** All three fields match between the two paths
(`name`, `handle`, `age`).

### dispatch_via_trait_matches_inherent_finalize
