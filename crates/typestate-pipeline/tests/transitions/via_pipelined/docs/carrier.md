### Carrier + Pipeline-integrated bag

`Hub` is the context type. `Author` is declared via `pipelined!`
(producing the `Pipelined<'a>` impl that the suite's
`#[transitions]` impls — which omit `error =` — read their error
type off).

Two `#[transitions]` impls cover one async-deferred transition
(`tag` on `Drafted`) and one sync-fallible (`publish` on
`Versioned`).

`Profile` is the Pipeline-integrated bag with no `error =`
attribute and no fallible setters. It declares
`pipeline(carrier = Author)` so the derive emits the bag's
setters directly on the carrier; the pipeline arms infer the
error type from the carrier's `Pipelined::Error`. `Profile`
lives in this module rather than next to the phase types because
the pipeline-arm codegen reaches into `Author`'s tuple-struct
internals — keeping them colocated is what makes that visibility
work.

Three test helpers wrap the (otherwise-private) `Author`
construction: `drafted` (Resolved-mode `Drafted`), `empty_profile`
(Resolved-mode empty `ProfileFactory`), and `drafted_inflight`
(InFlight-mode `Drafted` from a hand-built future).
