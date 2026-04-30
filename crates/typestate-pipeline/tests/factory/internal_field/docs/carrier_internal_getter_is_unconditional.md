## Carrier Internal Getter

**Invariant.** With `#[factory(pipeline(carrier = Author))]`, the
internal field's getter is also emitted on the carrier. Like the
standalone version, it has no flag bound — callable on any
Resolved-mode carrier shape.

**Failure mode this guards.** Symmetric to
`internal_getter_is_unconditional`, but on the carrier arm. If the
carrier-arm codegen forgot the "no bound" rule and stamped a
`Yes`-bound on the getter, it would only be callable after at
least one flag advanced.

**Setup.** Carrier with `namespace` set (via `new`); no other
setters called. Call `.namespace()` on the carrier directly.

**Assertion.** `carrier.namespace() == "op"`.

### carrier_internal_getter_is_unconditional
