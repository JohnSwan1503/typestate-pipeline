## Getter on `Yes` Flag

**Invariant.** A field's `bag.<field>()` getter is callable on bags
whose flag for that field is `Yes`, and returns `&FieldType` (or `&str`
for `String`).

**Failure mode this guards.** In safe mode, the bag's storage shape is
`<Flag as Storage<T>>::Out` — `T` for `Yes`, `()` for `No`. The
getter is implemented on the `Yes`-bound impl block; the
`Storage<T>::Out = T` projection must reach the right type. A bug here
would make the getter return `&()` (or fail to compile).

**Setup.** `UserBuilder` with `name` and `email` set, `age` left
unset. The getters are called on the partial bag without finalizing.

**Assertion.** `bag.name()` and `bag.email()` return the strings just
set.

### getter_borrows_set_field
