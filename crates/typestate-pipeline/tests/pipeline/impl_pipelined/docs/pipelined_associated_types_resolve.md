## Associated Types

**Invariant.** `impl_pipelined!(Author, ctx = Client, error = DummyError)`
emits a `Pipelined<'a>` impl whose associated types match what the
macro args declared:

- `Ctx = Client`
- `Error = DummyError`
- `Tag = ()` (the default when not specified)

The impl exists for both `Resolved` and `InFlight` modes — the trait
isn't mode-conditional.

**Failure mode this guards.** A regression in the macro's argument
parsing or trait emission could:

- Pick the wrong `Ctx` / `Error` / `Tag` (typo in the template).
- Emit the impl for only one mode (forgetting either `Resolved` or
  `InFlight`).
- Mix up associated-type names (e.g. emit `type Ctx = Pipeline`
  instead of `type Ctx = Client`).

The test uses generic functions with `where T: Pipelined<'a, Ctx = Client, ...>`
bounds; the bound can only be satisfied if the macro got every
associated type right.

**Setup.** Hand-rolled `Author` struct around `Pipeline<'a, Client,
(), S, DummyError, M>`. Two `assert_pipelined::<...>()` calls — one
for Resolved-mode, one for InFlight-mode.

**Assertion.** Both calls compile. The runtime body is empty; the
test is purely a compile-time witness.

### pipelined_associated_types_resolve
