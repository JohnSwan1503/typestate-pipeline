# Safety

`#[derive(TypestateFactory)]` generates code that uses three `unsafe`
operations by default. Each is gated by a type-level invariant:

1. **`MaybeUninit::assume_init_ref` in getters.** Each generated getter
   sits behind an impl bound that pins the field's flag to `Yes`. The
   flag is the type-level witness that the field was written by the
   corresponding setter.

2. **`MaybeUninit::assume_init_read` in `Drop`, `override_<field>`,
   `drop_<field>`, and `finalize`.** Each set field is read out into an
   owned stack temp before any user-defined `T::drop` runs. The temps
   then auto-drop, which gives Rust's panic-cleanup semantics: a
   panicking `T::drop` on one field still lets the remaining temps drop
   on unwind. (A naive sequence of in-place `assume_init_drop` calls
   would short-circuit at the first panic and leak the rest, since the
   surrounding `MaybeUninit` slots have no auto-drop fallback.)

3. **`ptr::read` + `ManuallyDrop` in setters / `finalize`.** Setters and
   `finalize` move fields out by `ptr::read`; `self` is wrapped in
   `ManuallyDrop` first so the original `Drop` does not run on
   moved-from `MaybeUninit` slots.

Two ordering invariants in the generated code are load-bearing:

**Setter ordering.** The (possibly fallible / async) transformer is
evaluated *before* `self` is wrapped in `ManuallyDrop`. If the
transformer fails (a `?` short-circuit) or its enclosing future is
dropped mid-await, `self` is still live and its normal `Drop` runs,
releasing every set field. If the order were inverted, an early exit
would leak the other set fields and leave the bag in a half-built state.

**`finalize` ordering.** All field reads land in stack locals *before*
any `default = …` expression is evaluated. A panic in a default thunk
then unwinds with every already-read field as an owned local that
auto-drops. Inlining the reads into the struct expression alongside
defaults would leak the fields *after* the panicking default, since
their `MaybeUninit` slots would still be sitting inside the
`ManuallyDrop`-wrapped `this`.

**`override_<field>` / `drop_<field>` ordering.** The OLD value is read
into a stack temp *and the new bag is constructed* before the temp's
auto-drop runs. A panic in the old value's `T::drop` therefore unwinds
with the new bag already in scope; the bag's panic-safe `Drop` reclaims
its other fields on the way out.

**Hygienic internal bindings.** The macro emits its phantom field as
`__tsh_markers` and its local bindings as `__tsh_this`,
`__tsh_field_value`, `__tsh_old_field`, `__tsh_new_bag`,
`__tsh_finalize_<field>`, and `__tsh_guard_<field>`. The `__tsh_`
prefix is unlikely to collide with a user-supplied field name or with
identifiers a user might reach for inside a `default = …` expression.
This prevents a user's struct from accidentally clashing with the
generated `_markers` field or shadowing the `this` binding that the
default expression's scope would otherwise expose.

**Explicit `Send` obligations on async pipeline arms.** The async
Resolved arm and the InFlight arm carry an explicit `where InputBag:
Send + 'a, OutputBag: Send + 'a` clause. When a user's field type is
non-`Send`, the diagnostic points at the impl block (where the user
can read the obligation) rather than at the depths of the
`Box::pin(async move { … })` instantiation.

**Sealed `Pipeline` fields.** `Pipeline::ctx` and `Pipeline::inner`
are private. The proc-macros destructure carriers with the public
[`Pipeline::into_parts`](crate::Pipeline::into_parts) /
[`Pipeline::ctx`](crate::Pipeline::ctx) accessors instead of
reaching into `self.0.ctx` / `self.0.inner`. This means a user's
carrier newtype cannot accidentally bypass the typestate machinery by
hand-substituting `inner` or forging a `_tag`/`_err` marker — the
fields aren't reachable from outside `typestate-pipeline-core`.

**Pinned bag layout.** The generated bag struct is annotated
`#[repr(Rust)]`. `MaybeUninit<T>` field reads via `ptr::read` rely on
default alignment; an accidental future `#[repr(packed)]` would
silently break that assumption. Pinning the representation makes any
layout change a deliberate edit rather than a quiet one.

**`PhantomData` marker tuple is always a tuple.** The bag's marker is
emitted as `PhantomData<( F1, F2, … )>` with a trailing comma after
*every* element. The trailing comma is load-bearing: with one flag it
produces the singleton tuple `(F,)` (not the parenthesised type
`(F)` which collapses to `F` itself), with zero flags it produces
`()`. The parenthesised single-type form would silently change the
bag's variance and auto-trait inheritance.

## How we test these properties

See the [`factory_no_leak`], [`factory_panic_safety`], [`factory_hygiene`],
[`factory_phantom_shape`], and [`factory_no_unsafe`] suites under
[`tests/safety/`].

[`factory_no_leak`]: https://github.com/JohnSwan1503/typestate-pipeline/blob/main/crates/typestate-pipeline/tests/safety/factory_no_leak.rs
[`factory_panic_safety`]: https://github.com/JohnSwan1503/typestate-pipeline/blob/main/crates/typestate-pipeline/tests/safety/factory_panic_safety.rs
[`factory_hygiene`]: https://github.com/JohnSwan1503/typestate-pipeline/blob/main/crates/typestate-pipeline/tests/safety/factory_hygiene.rs
[`factory_phantom_shape`]: https://github.com/JohnSwan1503/typestate-pipeline/blob/main/crates/typestate-pipeline/tests/safety/factory_phantom_shape.rs
[`factory_no_unsafe`]: https://github.com/JohnSwan1503/typestate-pipeline/blob/main/crates/typestate-pipeline/tests/safety/factory_no_unsafe.rs
[`tests/safety/`]: https://github.com/JohnSwan1503/typestate-pipeline/tree/main/crates/typestate-pipeline/tests/safety

Highlights of what those tests pin down:

- A failing fallible setter, a failing fallible overrider, and an async
  setter whose future is dropped mid-`await` all release every
  previously-set field (no `ManuallyDrop` leak).
- The bag's manual `Drop` reads each set field into an `Option<T>` stack
  guard, so a panicking `T::drop` on field N still drops fields N+1..end.
- `finalize()` reads all initialized fields into stack locals *before*
  evaluating any `default = …` expression, so a panic in a default
  unwinds with every already-read field auto-dropping.
- `override_<field>()` and `drop_<field>()` read the old value into a
  stack temp *and* construct the new bag before the temp's auto-drop
  runs — a panic in the old value's `T::drop` unwinds with the new bag
  already in scope.
- A struct with fields named `_markers`, `this`, `__field_value`,
  `__old_field`, and `__new_bag` compiles cleanly — internals are
  prefixed with `__tsh_` so they cannot collide.
- Bags with zero, one, and many flag generics all round-trip through
  `finalize`, and the trailing-comma `PhantomData<(F,)>` marker preserves
  `Send`/`Sync` auto-trait forwarding for the singleton case.
- A trybuild compile-fail case rejects reading `pipeline.ctx` or
  `pipeline.inner` directly (the privacy check), suggesting `ctx()` /
  `into_parts()` instead.

The `no_unsafe`-mode codegen path has a parallel coverage suite in
[`factory_no_unsafe`] that locks in the same safety guarantees without
`MaybeUninit`.

## The `no_unsafe` opt-out

Enabling the `no_unsafe` Cargo feature allows individual derives to opt
into a safe codegen path with `#[factory(no_unsafe)]`:

```toml
[dependencies]
typestate-pipeline = { version = "0.1", features = ["no_unsafe"] }
```

```rust,ignore
#[derive(TypestateFactory)]
#[factory(no_unsafe)]
struct User { /* fields */ }
```

The safe-mode bag swaps `MaybeUninit<T>` for `<Flag as Storage<T>>::Out`
— `T` when the flag is `Yes`, `()` when it is `No`. Each `(Yes, …)`
/ `(No, …)` flag combination is a structurally distinct sister type, so
no manual `Drop` is needed: setters write `T`, removers replace with
`()`, and Rust's auto-derived drop handles both shapes. `finalize()` for
optional-with-default fields uses the trait method `Storage::finalize_or`,
resolved at monomorphization rather than via a runtime `if`.

Without the feature, `#[factory(no_unsafe)]` is rejected at expansion
time so a downstream typo cannot silently cross codegen modes. The
attribute is *opt-in per derive* — turning the feature on does not
change the codegen of any existing derive.

`#[transitions]` and `pipelined!` / `impl_pipelined!` emit no `unsafe`
in either mode — they expand to safe glue around `Pipeline::resolved` /
`Pipeline::in_flight` / `Box::pin`.
