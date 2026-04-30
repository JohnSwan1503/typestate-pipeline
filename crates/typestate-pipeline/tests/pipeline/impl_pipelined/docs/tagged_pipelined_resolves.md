## Tagged Carrier

**Invariant.** `impl_pipelined!(... tag = MyTag)` emits a
`Pipelined<'a>` impl whose `Tag` associated type matches the
declared tag. Carriers can use the tag for type-level distinctions
(e.g. one carrier per dataset kind) without changing runtime data
shape.

**Failure mode this guards.** A regression that ignored the
`tag = ...` argument would emit `type Tag = ()` regardless of the
user's declaration. Two carriers meant to be distinct (one tagged
`MyTag1`, one tagged `MyTag2`) would collapse to the same type.

The test binds a `where T: Pipelined<'a, Tag = MyTag>` constraint
on a function and calls it with `Tagged<'_, Started, Resolved>`. The
bound is satisfiable only if the macro propagated the tag.

**Setup.** Hand-rolled `Tagged` carrier wrapping
`Pipeline<'a, Client, MyTag, S, DummyError, M>`, plus
`impl_pipelined!(Tagged, ctx = Client, error = DummyError, tag = MyTag)`.

**Assertion.** Compile-time witness only — the call to `assert::<...>()`
typechecks.

### tagged_pipelined_resolves
