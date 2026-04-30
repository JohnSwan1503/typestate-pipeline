## `bump_patch()` Increment

**Invariant.** `edit_existing_derived(reference) -> ... -> bump_patch()`
reads the prior version from the mock client, increments the patch
component, and threads the new `Reference` through the deploy
transition.

**Failure mode this guards.** Two separate failures could break this:

1. **Bump arithmetic wrong.** A regression that incremented the wrong
   semver component (minor or major instead of patch), or off-by-one,
   would surface as `0.1.2` or `0.2.0` instead of `0.1.1`.
2. **Bag-state read failure.** `bump_patch` is a transition body that
   reads from the carrier's prior version (which itself comes from the
   mock client's seeded state). A regression that lost the carrier's
   reference state, or that consulted the wrong client lookup, would
   produce either a panic (no prior version) or a different version.

**Setup.**

- The mock client is seeded with a derived dataset
  `(eth, transactions_opt)` already tagged at `0.1.0`.
- The author chain edits that existing dataset, adds a new table, and
  calls `bump_patch()` instead of `tag_version(...)`.

**Assertion.** `deployed.reference().version == Version::new(0, 1, 1)`
— exact patch increment from `0.1.0`.

### bump_patch_increments_existing_version
