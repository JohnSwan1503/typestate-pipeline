## Derived Flow Chain Folding

**Invariant.** The derived-dataset flow has a different bag and
different transitions from the evm-rpc flow, but the same chain-folding
property must hold: a sequence of async-deferred transitions stays in
`InFlight` for the whole chain and resolves at a single terminal
`.await?`.

**Failure mode this guards.** This is the second canonical happy path.
Compared to evm-rpc, the derived-dataset path exercises a different
factory bag (`add_dependency`, `add_table` setters that take typed
arguments instead of plain strings) and different transitions. If the
macro produced subtly different InFlight-arm signatures for the two
flows, the chain would fail to fold and force a mid-chain `.await`,
visible as a compile error on this test.

**Setup.** A default `Client` and a derived dataset with one
dependency entry and one table.

**Assertion.** `deployed.job_id().0 == 1` — first id from the mock
client. The point of the test is the *chain shape* (folded async
deferred transitions, terminal `.await?`), not the deployed reference.

### new_derived_flow_chains_through_single_await
