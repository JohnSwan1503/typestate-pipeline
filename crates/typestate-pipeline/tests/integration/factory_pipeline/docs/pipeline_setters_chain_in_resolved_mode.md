## Resolved Setter Chain

**Invariant.** Every setter / default helper / fallible setter shape
that the bag derive normally emits is also reachable directly from a
`Resolved` carrier. The user does not need `Pipeline::resolved` /
`into_state()` plumbing to drive a bag through its phases.

**Failure mode this guards.** If the codegen emitted only the
standalone bag arm and not the carrier arm, this test would not
compile — the user would have to write
`Author(Pipeline::resolved(server, factory.name(...)...))` for every
field, defeating the point of the integration. The test exercises the
fallible-setter Resolved-arm shape (`Result<Author<...>, AppError>`)
specifically, since the macro also has to lift the bag's `Result` to
the carrier's call-site shape correctly.

**Setup.** `empty_bag(server)` produces a fresh `Author<DatasetDataFactory, Resolved>`.
The chain sets `name`, `network`, and the fallible `label` (with leading
and trailing whitespace), expects the trim transformer to succeed, and
hands the bag to the `deploy` transition.

**Assertion.**

- `name == "ds-a"` — required setter went through.
- `parallelism == 4` — the field's flag was `No` at finalize, so the
  declared `default = 4` fired.
- `label == "primary"` — the trim transformer ran, dropping whitespace.
- `job_id == 1` — first ID from the server.

### pipeline_setters_chain_in_resolved_mode
