### Carrier + Pipeline-integrated bag

`Hub` is the context type, `Author` is the carrier (declared via
`pipelined!`), and `Job` is the bag — three required fields, one
of which is `internal`. The `pipeline(carrier = Author)` arm on
the bag emits the bag's setters / default helpers / getters
directly on the carrier.

`Job` lives in this module rather than next to other bags because
the derive's pipeline-arm codegen reaches into `Author`'s
tuple-struct internals — keeping them colocated is what makes
that visibility work without further plumbing.

`carrier(hub, namespace)` is the test entry point that wraps the
(otherwise-private) `Author` construction with the internal
`namespace` field already populated.
