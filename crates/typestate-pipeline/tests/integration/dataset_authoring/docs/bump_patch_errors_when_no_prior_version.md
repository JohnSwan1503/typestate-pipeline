## `bump_patch()` Without Prior Version

**Invariant.** `bump_patch()` against a dataset that's been registered
but never tagged surfaces the typed error
`AuthoringError::NoPriorVersion` at the chain's terminal `.await?`.

**Failure mode this guards.** Two regressions in particular:

1. **Silent default.** A buggy `bump_patch` could fall back to a
   default version (e.g. treating "no prior" as "start from 0.0.0
   and bump to 0.0.1"), making the failure invisible to the caller.
2. **Wrong error variant.** A code change that wired the bump path to
   a different error variant (e.g. `KindMismatch` or a generic
   `Internal`) would let the test pass under `Err(_)` matching but
   silently break callers who pattern-match on `NoPriorVersion`.

The test pins the *exact* error variant by `matches!`-ing on
`AuthoringError::NoPriorVersion`.

**Setup.** Register a derived dataset `(eth, untagged)` *without*
calling `client.tag(...)` afterward, so no version exists in the mock
client's tag map.

**Assertion.** The deploy chain returns
`Err(AuthoringError::NoPriorVersion)` at the terminal `.await`.

### bump_patch_errors_when_no_prior_version
