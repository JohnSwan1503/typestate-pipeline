### Defaults: relaxing the finalize bound

`#[field(default)]` (uses `T::default()`) or
`#[field(default = expr)]` (uses your expression) does two things
at once. It emits a `<field>_default()` helper that flips the
flag to `Yes` using the default expression, and it *relaxes the
finalize bound* so `finalize()` is callable whether the flag is
`Yes` or `No`. When the caller hasn't set the field, the default
expression is evaluated as the last step of `finalize()`.

The relaxation happens through the [`Satisfiable`](crate::Satisfiable)
trait: `finalize()`'s impl now requires `F: Satisfiable` for
the optional field instead of pinning it to the concrete `Yes`,
and both `Yes` and `No` implement `Satisfiable`. So the same
struct admits three call patterns — set explicitly, helper, or
skip entirely — all compiling, all producing a valid value.

Two restrictions to know about up front: `default` is rejected
on `async_fn` setters (the default has to be a synchronous
expression that `finalize()` can evaluate inline), and when
paired with [`input = T`](#setter-input-type-when-input--storage),
the default expression is typed as the *field* type, not the
input type — the helper bypasses the transformer.
