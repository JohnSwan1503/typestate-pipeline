## `pipelined!`-Supplied `IntoFuture`

**Invariant.** `pipelined!` emits an `IntoFuture` impl for the
InFlight-mode carrier. Awaiting an InFlight carrier yields the
corresponding Resolved-mode carrier with the awaited state.

**Failure mode this guards.** This is the sibling regression test to
[`tests::transitions::core::intofuture_resolves_inflight_back_to_resolved`](../core/index.html#intofuture_resolves_inflight_back_to_resolved),
but exercised against a hand-built InFlight carrier (rather than one
produced by an async-deferred transition). It pins that the
`IntoFuture` impl works regardless of how the InFlight carrier was
constructed.

**Setup.** Hand-build a `BoxFuture<'_, Result<Drafted, AppError>>`
that resolves to a `Drafted { name: "x" }`. Wrap it in an
`Author(Pipeline::in_flight(...))`. Call
`.into_future().await.unwrap()`.

**Assertion.** Yields `Author<Drafted, Resolved>` whose state's
`name == "x"` — the future ran, the carrier moved from InFlight to
Resolved.

### intofuture_provided_by_pipelined
