## Pipeline Arm With Internal

**Invariant.** With `#[factory(pipeline(carrier = Author))]`, the
non-internal fields still get pipeline-arm setters / default
helpers / overriders (whatever each field's attributes say). The
presence of an internal field doesn't suppress the pipeline arm
for the rest.

**Failure mode this guards.** A buggy codegen could "give up" on
the pipeline arm if any field is internal — emitting nothing for
the others either. The internal field's lack of a pipeline arm is
correct (an internal can't be set after construction); but the
non-internal fields must still get their arms.

The corresponding negative — that the internal field does *not*
get a pipeline arm — is pinned in
[`tests::ui::internal_field_no_pipeline_arm`](../../../ui/internal_field_no_pipeline_arm/index.html).

**Setup.** Open a `Resolved` carrier with the internal field
populated (`carrier(&hub, "eth")`). Drive the user-facing fields
via `parallelism(8).with_verify(true)`. Finalize through the
carrier.

**Assertion.** Finalized job has `namespace == "eth"` (set at
construction), `parallelism == 8`, `verify == true`. The
pipeline-arm chain typechecks at every step.

### pipeline_arm_works_for_non_internal_fields
