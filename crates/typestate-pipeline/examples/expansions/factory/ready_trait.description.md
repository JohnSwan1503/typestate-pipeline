### The Ready trait: generic over "finalize-callable"

A function that takes "any finalize-callable bag" runs into a
practical problem: the bag's flag tuple changes shape per
struct, and spelling it out explicitly is fragile. Adding a
field somewhere upstream forces every generic call site to add
a flag generic.

The derive's answer is a companion trait emitted alongside the
bag. For a bag named `UserFactory`, the trait is
`UserFactoryReady`, and it auto-impls on every flag combination
matching `finalize()`'s bounds. Generic code can write
`fn finalize_anything<B: UserFactoryReady>(bag: B) -> User` and
ignore the flag tuple entirely.

The trait method is `finalize` — the same name as the inherent.
That name choice is deliberate: a call site writing
`bag.finalize()` resolves to whichever is reachable, the
inherent on a concrete flag tuple or the trait method on a
generic `B: UserFactoryReady`. The implementation would recurse
back into itself if it called `self.finalize()` directly, so the
auto-impl body uses path-qualified syntax —
`<UserFactory<...>>::finalize(self)` — which Rust resolves to the
inherent (path-qualified resolution prefers inherent items over
trait items).

The trait's bounds depend on which codegen mode the derive uses.
Unsafe mode (the default) uses [`Satisfied`](crate::Satisfied)
and [`Satisfiable`](crate::Satisfiable); the `no_unsafe` mode
covered later uses [`Storage<T>`](crate::Storage) bounds. The
trait is identical at the call site either way — only the bound
names differ.
