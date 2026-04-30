### Safe-mode codegen: `no_unsafe`

The derive's default codegen path uses [`MaybeUninit`] to back
the bag's storage: each field is a slot that may or may not be
initialized depending on its flag, and the macro emits the
`unsafe { ptr::read(...) }` reads and the manual `Drop` impl
that this representation requires. The advantage is a single
struct shape that holds every flag combination — the typestate
machinery is the only thing that varies.

When the surrounding code (or audit policy) bans `unsafe` in the
crate, that's a problem. `#[factory(no_unsafe)]` opts an
individual derive into a parallel codegen path. Storage swaps
from `MaybeUninit<T>` to
[`<Flag as Storage<T>>::Out`](crate::Storage), which is `T` when
`Flag = Yes` and `()` when `Flag = No`. Each `(Yes, …)` /
`(No, …)` combination becomes a structurally distinct sister
type, the compiler-derived `Drop` handles both shapes, and there
is no `unsafe` anywhere in the emitted code.

Public method names and signatures are *identical* to the
unsafe path. Only the implementation differs. The one externally
visible signature difference is the `Storage` bound on the
`finalize`/Ready-trait impls (the unsafe path uses `Satisfied`
/ `Satisfiable` instead). At the call site, the two modes are
interchangeable.

The cost is codegen volume. The unsafe path has one struct
shape and one `Drop` impl regardless of how many flags the bag
carries; the safe path makes each `(Yes, …)` / `(No, …)` flag
combination its own monomorphization, with its own
auto-derived `Drop`. For a bag with *N* non-internal fields
that's `2^N` distinct sister types in the worst case (fewer in
practice, since only the combinations actually instantiated
get codegen). The trade is "one type and three `unsafe`
operations" against "many types and zero `unsafe`." Pick the
side your build policy prefers.

The attribute is gated on the `no_unsafe` Cargo feature.
Without the feature, `#[factory(no_unsafe)]` is rejected at
expansion time — a downstream typo cannot silently switch
codegen modes.

[`MaybeUninit`]: core::mem::MaybeUninit
