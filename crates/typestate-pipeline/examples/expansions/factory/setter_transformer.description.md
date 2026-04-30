### Setter transformer: in-setter normalization

By default, a setter does one thing: store the value. That's
fine when the caller already has the value in the right shape,
but real APIs often want a normalization step — trim whitespace,
lowercase an email, parse a string into a richer type — and
forcing every call site to do it themselves means that one
forgotten call site silently breaks a downstream invariant.

`#[field(setter = my_fn)]` runs `my_fn(val)` inside the setter
and stores its return. The setter's *signature* is identical to
the baseline — same input type, same output bag — only the body
changes. The transformer can do anything synchronous. When the
return type needs to differ from the input type, that's the
next recipe.
