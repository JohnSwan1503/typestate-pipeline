### Setter input type: when input ≠ storage

Sometimes the storage type has wrapping the call site shouldn't
have to spell out: `Option<T>`, `Box<T>`, smart pointers, or a
parsed type derived from a string. `#[field(input = T)]` —
paired with `setter = my_fn` — lets the setter accept a
different *input* type than the field stores. The transformer
bridges the gap: it takes `input`, returns the storage type, and
the setter signature surfaces the *input* type at the call site.

The interaction with `default = …` is the gotcha: the default
helper bypasses the transformer and writes the *field* type
directly. The default expression is evaluated as the storage
type, not the input type. That's deliberate — when the caller
opts out of supplying a value, they're opting out of the
transformer too — but worth remembering when typing the default.
