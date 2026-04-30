## Explicit Optional at `finalize`

**Invariant.** When an optional-with-default field is explicitly
set via `with_<field>(...)`, `finalize` reads the stored value, not
the default expression.

**Failure mode this guards.** The runtime `IS_SET` branch must pick
the right path. A buggy codegen that always evaluated the default
(or always read the slot) would surface here. The
`finalize_uses_defaults_when_optional_no` test pins the symmetric
case; together they prove the branch is per-field-correct.

**Setup.** `Configurable` with all three fields set explicitly:
`name("svc-b").with_parallelism(16).with_url("https://override.example".to_owned())`.

**Assertion.** `parallelism == 16` and
`url == "https://override.example"` — the explicit values, not the
declared defaults.

### finalize_keeps_explicit_values_when_optional_yes
