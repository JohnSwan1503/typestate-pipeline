## Zero-Flag Bag

**Invariant.** A struct in which every field is `#[field(internal)]`
has zero flag generics on the generated bag. The phantom marker
collapses to `PhantomData<()>`, the bag's `<...>` type parameter list
is empty, and `new(...)` takes the internals positionally. `finalize()`
must produce the original struct without any flag-related machinery.

**Failure mode this guards.** The macro template iterates over the
non-internal fields when building the marker tuple. If that iteration
isn't guarded against the empty case, the emitted code is
`PhantomData<()>` (correct, what we want) — but a bug like emitting
`PhantomData<>` or `PhantomData<,>` would break. This test exists as
a smoke check on the empty path.

The emitted bag looks like:

```text
struct AllInternalFactory { /* internals as plain T, no flag list */ }
impl AllInternalFactory {
    pub fn new(namespace: &'static str, version: u32) -> Self;
}
impl AllInternalFactory {
    pub fn finalize(self) -> AllInternal;
}
```

**Setup.** A struct with two `#[field(internal)]` fields. `new(...)`
takes them positionally; no setter chain.

**Assertion.** `finalize()` returns the populated struct with both
field values intact.

### all_internal_struct_finalizes_without_flag_generics
