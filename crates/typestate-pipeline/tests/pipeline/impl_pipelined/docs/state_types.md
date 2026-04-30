### State + tag markers

`Started` and `Finished` are phase-state markers — empty structs
that exist only as type parameters for the `State` slot of the
carrier. `MyTag` plays the same role in the carrier's `Tag` slot,
exercising `impl_pipelined!`'s optional `tag = MyTag` argument.

None carry runtime data; the smoke tests only check that the
trait machinery compiles around them.
