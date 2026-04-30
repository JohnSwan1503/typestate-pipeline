## Field Names Match Macro Internals 

**Invariant.** A user struct whose field names happen to coincide with
identifiers the macro uses internally must compile and round-trip
through `finalize` like any other struct. Hygiene is a black-box
contract: identifiers the macro chooses are private to the macro and
must not collide with names a user could plausibly pick.

**Failure mode this guards.** The previous codegen used unprefixed
identifiers like `_markers`, `this`, `__field_value`, `__old_field`,
and `__new_bag` for its internal bindings and the bag's phantom field.
A user struct with a field named `_markers` produced a duplicate-field
error on the generated bag; one with a field named `this` could shadow
the macro's `let this = ...` inside a `default = …` expression and
silently break.

The fix prefixes every macro-internal identifier with `__tsh_`
(`__tsh_markers`, `__tsh_this`, `__tsh_field_value`, `__tsh_old_field`,
`__tsh_new_bag`, `__tsh_finalize_<field>`, `__tsh_guard_<field>`). The
`__tsh_` prefix is unlikely to collide with anything a downstream user
would write, while remaining short enough to keep error messages
readable.

**Setup.** `CollidingFieldNames` declares one field per previously-colliding
internal binding, including two `overridable, removable` fields named
`__old_field` and `__new_bag` (the names of the override/remove stack
temps). The test exercises every emitted method — setter, getter,
finalize — so a regression would surface as either a compile error or
a runtime mismatch.

**Assertion.** Round-trip through `new() -> setter chain -> finalize`
preserves every field's value. The test compiling at all is most of the
proof; the runtime checks just confirm the names didn't get mangled in
some other way.

### struct_with_field_names_matching_macro_internals_compiles`
