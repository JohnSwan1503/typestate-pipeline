## EVM-RPC Happy Path

**Invariant.** A fresh evm-rpc dataset can be authored end-to-end —
register fields, set a version, configure deploy parameters, deploy —
with the entire chain folding into a single terminal `.await?`. The
deployed result carries the expected `Reference` and the first
`job_id` from the mock client.

**Failure mode this guards.** This is the canonical happy-path
regression for the dataset-authoring chain. Any of the following
would break it:

- A factory bag's flag generic mismatching the
  `#[transitions]` impl bound it's eventually consumed by (chain
  doesn't typecheck).
- A `register()` transition that drops or rewrites the namespace/name
  internal fields (assertion on `Reference` fails).
- The mock `Client`'s `next_id` not incrementing (assertion on
  `job_id == 1` fails).
- A regression in chain-folding that forces the user to `.await?` mid-
  chain instead of once at the end (test compiles differently — likely
  doesn't compile because the chain shape no longer matches).

**Setup.** A default `Client` (no pre-seeded datasets, fresh atomic
counters) and the canonical `(eth, blocks)` namespace/name pair.

**Assertion.** `deployed.job_id().0 == 1` and `deployed.reference()`
matches `(eth, blocks, 0.1.0)`.

### new_evm_rpc_flow_terminates_at_deployed
