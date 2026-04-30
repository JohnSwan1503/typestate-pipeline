## Wrong Kind Error

**Invariant.** `edit_existing_derived(reference)` against a dataset
registered as `evm_rpc` (rather than `derived`) surfaces the typed
error `AuthoringError::KindMismatch { expected: "derived", .. }`
during the `register()` step.

**Failure mode this guards.** Kind mismatch is precisely the situation
you want a typed pipeline to catch — silently downcasting an evm-rpc
dataset's body to a derived shape would corrupt downstream state. The
test pins three things:

1. The kind check happens at the `register()` transition, *not* later
   at deploy.
2. The error carries the kind that was *expected* (the kind the entry
   point declared, here `"derived"`), so the caller knows what they
   asked for.
3. The error is structured (`AuthoringError::KindMismatch { ... }`),
   not a stringly-typed `Err(format!(...))`. Pattern-matching on the
   variant survives refactors better than string equality.

**Setup.** Register `(eth, blocks)` as `evm_rpc`. Then build an
`edit_existing_derived` chain pointing at the same name. The
`register()` step has to reject before any further transition runs.

**Assertion.** `result` is `Err(AuthoringError::KindMismatch {
expected: "derived", .. })`. The `..` lets the test ignore other
fields the error variant carries (the actual kind the dataset *was*),
so future additions to the variant don't break this assertion
spuriously.

### edit_existing_kind_mismatch_surfaces_error
