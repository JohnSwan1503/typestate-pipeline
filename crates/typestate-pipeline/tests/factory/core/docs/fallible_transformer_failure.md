## Fallible Transformer Failure

**Invariant.** A `setter = …, fallible` setter that returns `Err`
propagates the error to the caller and does not advance the bag's
state. The original bag is consumed (per move semantics), but no
flag transition fires and the failing input is dropped inside the
transformer (since the transformer owns it on entry and chose to
drop it on the failure path).

**Failure mode this guards.** A buggy codegen could:

- Swallow the error and silently advance the flag with whatever
  garbage the transformer left in storage.
- Move forward as `Ok(bag)` even though the transformer returned
  `Err`.

The leak-on-failure side of the contract is pinned in
[`tests::safety::factory_no_leak::fallible_setter_failure_drops_other_set_fields`](../../safety/factory_no_leak/index.html#fallible_setter_failure_drops_other_set_fields);
this test only confirms the error variant + value propagates.

**Setup.** `ValidatedUser` whose setter rejects empty input.
`name(String::new())` is called.

**Assertion.** The setter returns `Err(ValidationError(msg))`
where `msg == "name is empty"` — exact variant and exact message.

### fallible_transformer_failure
