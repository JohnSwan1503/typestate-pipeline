## Bag-Finalize In Chain

**Invariant.** A factory bag held as a phase's state can be finalized
inside a `#[transitions]` body, with the resulting value flowing
naturally into the next phase. Sync-fallible (`Result`) and
async-deferred transitions chain across that boundary, folding into a
single terminal `.await?`.

**Failure mode this guards.** Two failure modes:

1. **Bag-pipeline coupling break.** Earlier sketches kept the bag and
   the carrier as fully separate type universes — the user had to
   manually `finalize()` outside the chain and re-enter the carrier
   with the result. That defeats the point of composing the two
   patterns. The transition body here finalizes the bag *inside* the
   chain, with the macro's GAT projection providing the next-phase
   carrier type seamlessly.
2. **Sync fallible + async deferred fold mismatch.** `submit` returns a
   sync `Result`; `confirm` is async. If the codegen handled the two
   bodies as independent arms (without sharing the InFlight folding
   logic), the `Result` from `submit` would force the caller to
   `.expect()` *before* `confirm`'s future could be appended. This
   test deliberately writes `submit().expect(...).confirm().await?` to
   confirm the sync-fallible Resolved-arm yields a `Result<…, E>` at
   the call site (so we can `expect()` synchronously) and `confirm`'s
   InFlight arm folds into the chain after.

**Setup.** A `Server` and a `User` with all required fields populated
non-empty. `drafting(...)` builds the carrier in `Drafting` mode.

**Assertion.** The terminal carrier's state has `user.name == "Alice"`
and `confirmation_id == 1` (the server's first id).

### full_chain_bag_into_pipeline
