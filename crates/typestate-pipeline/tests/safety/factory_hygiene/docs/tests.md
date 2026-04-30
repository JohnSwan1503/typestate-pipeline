# Hygiene regression tests

Pins the rename of macro-internal bindings to a `__tsh_` prefix so a
user struct's field names — including names that the previous
generated code used internally (`this`, `_markers`, `__field_value`,
`__old_field`, `__new_bag`) — no longer collide with the bag.

Each `#[derive(TypestateFactory)]` here would have failed to compile
under the old codegen because the macro emitted a `_markers:
PhantomData<…>` field next to a user-declared `_markers`, or
introduced a `let this = …` that the user's `default = …` expression
could shadow.

These tests are pure compile-passes; the runtime assertions just
confirm the round-trip is intact.
