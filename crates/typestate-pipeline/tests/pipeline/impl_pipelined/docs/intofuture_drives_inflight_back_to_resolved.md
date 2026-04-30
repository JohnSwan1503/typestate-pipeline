## `IntoFuture` Forwarding

**Invariant.** `impl_pipelined!` emits an `IntoFuture` impl on the
InFlight-mode carrier so users can `.await` the InFlight result and
get back a Resolved carrier of the same state.

**Failure mode this guards.** Same shape as the corresponding
`pipelined!` test — but on a hand-rolled struct rather than the one
`pipelined!` declares. If `impl_pipelined!`'s codegen for `IntoFuture`
diverged from `pipelined!`'s (e.g. wrong bound, wrong return type),
this case would surface the divergence.

**Setup.** Hand-build a `BoxFuture<'_, Result<Started, DummyError>>`
that resolves to `Started`. Wrap it in
`Author(Pipeline::in_flight(...))`. Call `.await`.

**Assertion.** Yields `Author<Started, Resolved>` — the type
annotation forces the Resolved-mode shape. The body is otherwise
empty (we don't assert on `Started`'s value because it carries no
data).

### intofuture_drives_inflight_back_to_resolved
