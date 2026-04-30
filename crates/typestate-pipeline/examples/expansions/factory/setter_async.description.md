### Async setter: I/O in the setter

`fallible` lets the setter *reject* an input synchronously.
`#[field(setter = my_fn, async_fn)]` lets it *await* one. The
setter becomes `async fn`, awaiting the transformer before
constructing the next bag. The flag flips after the future
resolves.

Combine with `fallible` for an async fallible setter: the
return becomes `Result<NextBag, Error>` after `.await`. That's
the shape to reach for when validation needs a remote round-trip
— hashing a password, looking up a value, calling a normalizer
service.

The single restriction: `default` is rejected when paired with
`async_fn`, because defaults must be synchronous expressions
that `finalize()` can evaluate inline. An async default would
turn `finalize()` itself into an `async fn`, and that ambiguity
is rejected at expansion rather than papered over with a
runtime check.
