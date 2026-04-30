## Fallible Setter Failure

**Invariant.** When a fallible setter rejects, the input bag's other
set fields are released. Mirrors the unsafe-mode `factory_no_leak`
test of the same name, but on the safe codegen path.

**Failure mode this guards.** The safe-mode path doesn't have the
`ManuallyDrop` hazard the unsafe path had; the bag's normal auto-Drop
covers field cleanup as long as `?` short-circuits *before* `self`
gets consumed into the new sister-struct literal. This test pins
that ordering: the transformer evaluates first, and only on `Ok` does
the setter touch `self`.

**Setup.** `SyncFallibleBag` with `other` set, then `main` called
with a transformer that drops its input and returns `Err(Reject)`.

**Assertion.** After the failing setter:

- `result.is_err()`.
- `alive() == baseline` — `other` was released by the bag's auto-Drop
  when `?` returned, and the failing setter's input (`m`) was dropped
  inside the transformer.

### fallible_setter_failure_drops_other_set_fields
