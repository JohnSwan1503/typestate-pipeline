## Fallible Overrider Failure

**Invariant.** When a fallible `override_<field>` rejects, the bag is
left intact (with its old value) and the failing new value is
dropped inside the transformer. The bag's other fields stay in their
prior state and only release when the bag itself is dropped.

**Failure mode this guards.** Same shape as the setter failure, but
on the override path which is more complex (needs to reach into a
`Yes` slot and replace it). Pins that the override codegen also
runs the transformer *before* committing to the new bag literal —
otherwise the old value would have already been moved out, leaving
an irrecoverable hole.

**Setup.** `OverridableBag` with `other` and `m_old` (initial set
succeeds because `"m_old"` ≠ `"fail"`). Then `override_main` is called
with `Counted::new("fail")`, which the transformer drops and rejects.

**Assertion.** After the failing override:

- `result.is_err()`.
- `alive() == baseline` — `other` and `m_old` are still owned by
  `bag` (which then drops at end of inner block, releasing both).
  The failing new value `fail` was dropped by the transformer.

### fallible_overrider_failure_drops_other_set_fields
