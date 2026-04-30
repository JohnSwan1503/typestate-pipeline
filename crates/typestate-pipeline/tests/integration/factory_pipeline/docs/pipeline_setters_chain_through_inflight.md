## InFlight Setter Chain

**Invariant.** The carrier-arm setters are emitted for both `Resolved`
and `InFlight` modes. On `InFlight`, fallible setters fold their
`Result` into the chained pending future — there's no `Result` at the
call site; the error surfaces at the chain's terminal `.await?`.

**Failure mode this guards.** Without an InFlight arm, you'd have to
`.await` the carrier between every setter to step out of the pending
future, set the field, and re-enter — defeating chain-folding.
Concretely, the InFlight arm has to satisfy `Send + 'a` bounds on the
input/output bag types and box-pin the chained future, and a buggy
codegen could either omit the arm entirely (compile error here) or
lift the `Result` to the call site instead of folding it (different
compile error).

The test specifically uses the fallible setter (`label`) on `InFlight`
to confirm the `Result` is folded, not surfaced.

**Setup.** Construct the InFlight carrier by hand (none of this
suite's transitions naturally open an InFlight chain — the only async
transition is the terminal `deploy`). The hand-built future yields
`Ok(empty_factory)` so the bag arrives at the first setter clean.
Then `name -> network -> label("  alpha  ")` chains in InFlight,
folding the trim transformer's `Result` into the pending future.

**Assertion.** Awaiting the chain yields a Resolved bag whose
`label == "alpha"` (whitespace trimmed by the transformer that ran
inside the folded future).

### pipeline_setters_chain_through_inflight
