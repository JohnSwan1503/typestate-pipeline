# `#[transitions]` body-shape coverage

The `#[transitions]` macro recognizes four body shapes and emits a
different Resolved/InFlight arm pair for each. This suite walks
through a slice of the `amp client-admin::author` pipeline —
`Registered → Versioned → JobConfigured → Deployed` — exercising
every shape exactly once:

| Body shape | Where in the chain |
|---|---|
| sync infallible | `with_parallelism` (Versioned → JobConfigured) |
| sync fallible | `validate_and_finalize` (JobConfigured → JobConfigured) |
| async deferred | `tag_version` (Registered → Versioned), `deploy` (JobConfigured → Deployed) |
| async breakpoint | `confirm_and_tag` (Registered → Versioned) |

The tests pin five separate properties: full chain folding into a
single terminal `.await?`, breakpoint shapes that force explicit
awaits mid-chain, sync-fallible Resolved arms returning `Result`
directly, sync-fallible folding through InFlight chains, and the
carrier's `IntoFuture` impl driving InFlight back to Resolved.

The same `Author` carrier and the same domain types power every
test, so the chain shape variations are isolated to the test bodies
themselves.
