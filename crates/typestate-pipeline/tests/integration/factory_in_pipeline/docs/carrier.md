### Carrier + transitions

`pipelined!(pub Author, ctx = Server, error = SubmitError)`
declares the carrier. The two `#[transitions]` impls fold
`Drafting -> Submitted` (sync fallible — finalize the bag,
validate the resulting `User`) and `Submitted -> Confirmed`
(async deferred — round-trip to the server). Together they
exercise the two body shapes most relevant to bag/pipeline
composition.

`drafting(server, user)` is the test entry point that builds a
fully-set bag from a plain `User` and walks into the carrier in
`Drafting` mode. Real code would build the bag interactively via
setters; here we round-trip a `User` through the bag for
compactness.
