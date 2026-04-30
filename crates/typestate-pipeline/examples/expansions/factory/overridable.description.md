### Overridable: replacing a set value

`#[field(overridable)]` is the in-place sibling of `removable`.
It emits an `override_<field>(self, val)` method that consumes
a `Yes`-flagged bag, drops the prior value, stores the new
value, and returns a bag whose flag is *still* `Yes`. The bag
never visits the `No` shape between the two values.

Compare the two halves of the same flag transition: a setter
takes `No → Yes` (consumes an unset bag), `override_` takes
`Yes → Yes` (consumes a set bag), and `drop_` takes `Yes → No`.
Together they cover every direction of the flag.

`overridable` composes with the setter knobs covered later —
`fallible`, `async_fn`, `setter = my_fn` — exactly the same way
the regular setter does. The override emits its own arms;
behavior is symmetric.
