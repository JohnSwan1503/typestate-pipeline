## Default at `finalize`

**Invariant.** A field with `default = None` and `input = String`
left unset (flag still `No`) finalizes with `worker == None`. The
default expression is evaluated at finalize time, not at setter
time, and it's typed as the storage type so the bypass logic isn't
even involved here — `finalize`'s default branch just writes the
expression's value to the field slot.

**Failure mode this guards.** This is the symmetric "default at
finalize" path. Together with
[`default_helper_bypasses_transformer`](#default_helper_bypasses_transformer),
the two tests cover both default routes:

- `<field>_default()` helper — explicit user call.
- `finalize()` evaluating the default expression — implicit on
  unset optional.

If the codegen tangled these paths (e.g. routed `finalize`'s default
branch through the transformer), the storage type / input type
mismatch would surface here.

**Setup.** Same `Profile` bag. Chain: `name("carol").finalize()` —
no `worker` setter call.

**Assertion.** `profile.worker == None` — finalize evaluated the
default.

### unset_default_field_uses_default_at_finalize
