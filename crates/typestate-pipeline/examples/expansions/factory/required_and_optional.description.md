### Required vs. optional: setter naming

The first knob — `#[field(optional)]` — changes the *name* of
the setter from `name(val)` to `with_name(val)`. That's all it
does at the type level. The flag still has to be `Yes` for
`finalize()` to compile, just like a `required` field.

The naming convention isn't load-bearing — it's a hint to the
reader of the call site that the field isn't a core input. The
real way to make a field genuinely optional, where `finalize()`
can succeed without the field ever being set, is the next
recipe.
