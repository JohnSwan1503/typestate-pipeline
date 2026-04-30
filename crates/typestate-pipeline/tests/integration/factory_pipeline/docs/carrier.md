### Carrier + Pipeline-integrated bag + transition

`Author` is the carrier (declared via `pipelined!`). `DatasetData`
is the bag — it deliberately exercises four different
`#[field(...)]` shapes (`required`, `required + removable`,
`default + overridable`, `required + setter + fallible`) so the
generated pipeline arm spans every method kind.

`DatasetData` lives in this module rather than next to other
domain types because the derive's `pipeline(carrier = Author)`
arm reaches into `Author`'s tuple-struct internals — keeping the
two definitions colocated is what makes that visibility work.

The `#[transitions]` impl declares `deploy` on the *exact* flag
tuple `(Yes, Yes, No, Yes)` — the strict bound is intentional,
demonstrating that a transition can operate on a specific bag
shape, with the type system rejecting any call site whose bag
doesn't match.

`empty_bag(server)` and `empty_inflight_bag(server)` are the test
entry points wrapping the (otherwise-private) carrier
construction in Resolved and InFlight modes respectively.
