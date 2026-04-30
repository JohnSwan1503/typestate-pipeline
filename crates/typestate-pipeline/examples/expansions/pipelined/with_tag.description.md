### Phantom tag: distinguishing same-shape carriers

Sometimes two carriers have the same context, the same error,
the same phase types — they're *structurally* identical — but
they should not be interchangeable. A pipeline for user-facing
datasets and one for internal-only datasets might pass the same
ctx and walk the same phase progression, but a function that
expects one should refuse the other.

A phantom tag turns "structurally identical" into "type-level
distinct" without changing the value-level shape. The tag is
invariant in the type system but doesn't carry runtime data.
`pipelined!(…, tag = MyTag)` (or `impl_pipelined!(…, tag = MyTag)`)
attaches one. When omitted, the tag defaults to `()`.

The tag is exposed as the `Tag` associated type on the
[`Pipelined<'a>`](crate::Pipelined) impl, so generic code can
branch on it (or constrain a bound) at the type level. That's
also how `#[transitions]` could, in principle, refuse a
transition impl for the wrong tag — though in practice most
transitions are written against a single carrier and the
distinction matters most at *consumer* boundaries.
