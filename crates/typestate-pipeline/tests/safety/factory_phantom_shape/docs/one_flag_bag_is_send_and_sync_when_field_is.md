## Singleton Auto-Trait Forwarding

**Invariant.** A `OneFlagFactory<F>` bag is `Send` and `Sync` whenever
its single field is. The auto-trait inheritance must travel through
the `PhantomData<(F,)>` marker as a tuple — i.e. the same way a
`PhantomData<(F1, F2, F3)>` would forward auto-traits in the many-flag
case.

**Failure mode this guards.** This test is what would catch the
trailing-comma bug that the round-trip test misses. If the macro
template emitted `PhantomData<(F)>` (no comma), Rust would parse it as
`PhantomData<F>` — semantically identical for *behavior* (the bag
still finalizes the same), but the auto-trait inheritance now goes
*directly through `F`* instead of through a tuple wrapper. For one
flag that's the same Send/Sync verdict; the bug would lurk silently.

The reason the bug *would* matter:

- `Send`/`Sync` *happen* to be inherited identically through `(F,)`
  and `F` — they're auto-traits and tuples auto-impl them when their
  members do.
- *Variance* is not. `PhantomData<F>` makes the bag invariant in `F`;
  `PhantomData<(F,)>` does too — but a future macro change might add
  more sophisticated variance carriers (e.g. `PhantomData<(F, fn() -> F)>`)
  where the difference between "inside the tuple" and "directly the
  type parameter" matters.

So this test is *belt-and-braces*: it checks the auto-trait inheritance
that the singleton case shares with the many-flag case, locking the
intent of the trailing comma even though the regression itself isn't
directly observable through `Send`/`Sync`.

**Setup.** A `struct OneFlag { name: String }` (its single field is
`Send + Sync`). The test uses generic `assert_send::<T>()` /
`assert_sync::<T>()` helper functions to compile-check.

**Assertion.** `OneFlagFactory<Yes>: Send + Sync` and
`OneFlagFactory<No>: Send + Sync`. If either fails, the test doesn't
compile.

### one_flag_bag_is_send_and_sync_when_field_is
