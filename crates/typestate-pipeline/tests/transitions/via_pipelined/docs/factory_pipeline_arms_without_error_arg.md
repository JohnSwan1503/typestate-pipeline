## Inferred Error On Factory Arms

**Invariant.** A `TypestateFactory` derive with
`#[factory(pipeline(carrier = Author))]` infers the carrier's error
type from `Pipelined::Error` — no `error =` needed at the factory
attribute when there are no fallible setters. The generated pipeline
arms (setters, default helpers) on the carrier compile and run
correctly.

**Failure mode this guards.** Two failure modes:

1. **Missing GAT path for factories.** The factory derive has its
   own resolution path for the pipeline-arm error type. A regression
   that worked for `#[transitions]` (the previous test) but failed
   for `#[factory(...)]` would surface here.
2. **Conservative pessimism.** A buggy codegen could *require*
   `error = …` even when there are no fallible setters, breaking
   ergonomics for the common no-fallible case.

**Setup.** `Profile` derives `TypestateFactory` with
`#[factory(pipeline(carrier = Author))]`, no `error =`. Two fields:
`handle` (required) and `age` (default 0). Both are non-fallible.
The carrier opens with an empty bag, then chains
`handle("alice").age_default()` via the pipeline-integrated arms.

**Assertion.** Finalized profile has `handle == "alice"` and
`age == 0` (the default).

### factory_pipeline_arms_without_error_arg
