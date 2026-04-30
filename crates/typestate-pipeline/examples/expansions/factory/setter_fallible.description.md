### Fallible setter: validating in the setter

The transformers covered so far have all been infallible — they
take a value and produce a (possibly different) value. Real APIs
sometimes need to *reject* the input: an empty name, a malformed
email, a number outside the allowed range. The validation has to
happen somewhere, and "in the setter, before the bag is mutated"
is the right place when the caller has the input on hand and
wants the error directly.

`#[field(setter = my_fn, fallible)]` makes the setter return
`Result<NextBag, Error>`. The error type is read from
`#[factory(error = Error)]` — required at the bag level when any
field is `fallible`. A failing setter does not consume `self`
past the point of failure: the bag still owns its previously-set
fields, and they drop normally on the `Err` branch. No leak,
even on the unhappy path.

The leak guarantee is exhaustively tested in the
[`tests::safety::factory_no_leak`](crate::tests::safety::factory_no_leak)
suite — every fallible-setter shape is exercised against
panicky and `Err`-returning transformers, and the leak counters
land at zero. It's a hard invariant, not a best-effort claim.
