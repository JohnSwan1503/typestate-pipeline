# typestate-pipeline

Compile-time-checked typestate scaffolding for Rust: a dual-mode pipeline
carrier and a named-field accumulator derive that compose for both
cross-phase state machines and within-phase argument bags.

`typestate-pipeline` ships three macros and a runtime carrier:

- [`#[derive(TypestateFactory)]`](#derivetypestatefactory) — sibling
  factory type with one flag generic per field.
- [`#[transitions]`](#transitions) — Resolved + InFlight method pairs from
  a single source body on a `Pipeline` newtype.
- [`pipelined!`](#pipelined--impl_pipelined) /
  [`impl_pipelined!`](#pipelined--impl_pipelined) — declarative shorthand
  for the conventional carrier shape.

The macros are independent (either is useful on its own) but compose: a
factory can run *inside* a pipeline phase, and the
`#[factory(pipeline(carrier = …))]` arm even emits its setters directly on
the user's carrier.

## Workspace layout

```text
typestate-pipeline          # facade — depend on this
├── typestate-pipeline-core    # runtime: Pipeline, Mode, Pipelined, flag traits
└── typestate-pipeline-macros  # proc-macros (use through the facade)
```

The proc-macros emit fully-qualified paths through
`::typestate_pipeline::__private::*`, so always depend on the facade
crate; depending on the macros crate alone produces unresolved paths.

## Mental model

Two orthogonal axes. Each macro operates on one of them, and they
compose freely.

### Per-field flag — `#[derive(TypestateFactory)]`

Every non-internal field on a derived bag carries a flag generic that is
either `No` (unset) or `Yes` (set). Setters flip `No → Yes`; `finalize()`
resolves only when every required flag is `Yes`. The full transition
graph, covering every `#[field(…)]` mutability attribute:

- `Factory::new()` / `Default::default()` — initial state; every flag `No`.
- `.field(val)` setter (required field, default naming) — `No → Yes`.
- `.with_field(val)` setter (`optional` or `default`) — `No → Yes`.
- `.field_default()` helper (`default = …`) — `No → Yes`, using the
  declared default expression.
- `.drop_field()` (`removable`) — `Yes → No`; drops the stored value.
- `.override_field(val)` (`overridable`) — `Yes → Yes`; drops the old
  value before storing the new one.
- `.finalize()` — consumes the bag once every required flag is `Yes`
  (optional-with-default flags may be either).

`#[field(internal)]` fields don't appear in the flag-generic list at all
— they're set positionally on `new(…)` and have an unconditional getter.

### Carrier mode — `#[transitions]`, `pipelined!`, `impl_pipelined!`

The pipeline carrier is dual-mode: `Resolved` holds the current state
directly, `InFlight` holds a pending future that will yield the next
state. Each `#[transition]` body shape picks which arrow it takes; the
two arms emitted per transition (one per starting mode) end up in
different places:

- **Sync infallible** (`fn` returning a non-`Result`):
  `Resolved → Resolved`, `InFlight → InFlight`.
- **Sync fallible** (`fn` returning `Result<_, E>`):
  `Resolved → Result<Resolved, E>` (handle at the call site),
  `InFlight → InFlight` (the `Result` folds into the pending future).
- **Async deferred** (`async fn`, the default for async bodies):
  `Resolved → InFlight` (lifts the chain), `InFlight → InFlight`.
- **Async breakpoint** (`#[transition(deferred = false)]`): both arms
  return `async fn → Result<Resolved, E>`; the caller must `.await?` here.

Crosscutting: any `InFlight` carrier `.await?`s into a `Resolved` of the
same state via the carrier's `IntoFuture` impl.

### Chain folding

A run of async-deferred transitions stays in `InFlight` for the whole
chain and resolves at a single terminal `.await?`:

```rust
Author<Registered, Resolved>    // start
    .tag_version(7)             // async deferred  -> lifts to InFlight
    .with_parallelism(8)        // sync infallible -> folds into pending
    .deploy()                   // async deferred  -> folds into pending
    .await?                     //                 -> resolves to Author<Deployed, Resolved>
```

Adding `#[transition(into = …, deferred = false)]` to one of the steps
(an *async breakpoint*) forces the chain to `.await?` at that step,
landing back in `Resolved` for whatever follows.

## `#[derive(TypestateFactory)]`

Generates `<Name>Factory<F1, F2, …>` with one flag generic per field.
Setters consume `self` and transition the relevant flag from `No` to
`Yes`. `finalize()` is callable only when every required flag is `Yes`.

```rust
use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct User {
    #[field(required)]
    name: String,
    #[field(required)]
    email: String,
    #[field(default = 18)]
    age: u32,
}

let user = UserFactory::new()
    .name("Alice".into())
    .email("alice@example.com".into())
    .with_age(30)        // optional fields → `with_<field>`
    .finalize();
```

Selected features:

- `setter = my_fn` + `fallible` / `async_fn` — call a transformer inside
  the setter (sync, fallible, async, or async-fallible).
- `default` / `default = expr` — declare a default; emits a
  `<field>_default()` helper. Optional-with-default fields can finalize in
  either flag state.
- `removable` — emit `drop_<field>(self)` reverting the flag to `No`.
- `overridable` — emit `override_<field>(self, val)` on `Yes`-flagged bags.
- `internal` — set positionally at `new(…)`, locked from then on.
- `pipeline(carrier = …)` — also emit Resolved + InFlight method pairs
  on the carrier.
- `finalize_async(via = …, into = …)` — async finalize hook.

See [the proc-macro docs](crates/typestate-pipeline-macros/src/lib.rs) for
the full attribute reference.

## `#[transitions]`

Decorates an `impl` block on a tuple-struct newtype around `Pipeline`.
Each method marked `#[transition(into = NextState)]` is expanded into a
Resolved + InFlight method pair from a single source body. Four body
shapes are recognized: sync infallible, sync fallible, async deferred
(default for `async fn`), and async breakpoint (`deferred = false`).

```rust
use typestate_pipeline::{pipelined, transitions};

pipelined!(Author, ctx = Client, error = AuthoringError);

#[transitions]
impl<'a> Author<'a, Registered> {
    #[transition(into = Versioned)]
    pub async fn tag_version(state: Registered, ctx: &Client, version: u32)
        -> Result<Versioned, AuthoringError>
    {
        ctx.tag(state.name.clone(), version).await;
        Ok(Versioned { name: state.name, version })
    }
}

// chain folds into a single terminal `.await?`
let v = author.tag_version(7).deploy().await?;
```

The destination type is read off the carrier's `Pipelined<'a>` impl as a
GAT projection (`<Self as Pipelined<'a>>::Resolved<NextState>`), so
carriers with extra generics or unusual ordering keep working as long as
the trait impl is correct. Generated transition code uses **no** `unsafe`.

## `pipelined!` / `impl_pipelined!`

Declarative shorthand for the conventional carrier shape
(`<'a, S, M = Resolved>` tuple-struct newtype around `Pipeline`):

```rust
// declares the carrier struct + Pipelined impl + IntoFuture forwarding
typestate_pipeline::pipelined!(pub Author, ctx = Client, error = AuthoringError);

// alternative: hand-write the struct (custom derives, extra generics, …)
// and emit only the trait impls
typestate_pipeline::impl_pipelined!(Author, ctx = Client, error = AuthoringError);
```

Both also emit a chainable `inspect(|carrier| …)` combinator on Resolved
and InFlight.

## Safety

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
[`Pipeline::into_parts`] / [`Pipeline::ctx`] accessors instead of
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

[`Pipeline::into_parts`]: https://docs.rs/typestate-pipeline-core/latest/typestate_pipeline_core/pipeline/struct.Pipeline.html#method.into_parts
[`Pipeline::ctx`]: https://docs.rs/typestate-pipeline-core/latest/typestate_pipeline_core/pipeline/struct.Pipeline.html#method.ctx

### How we test these properties

<table>
    <thead>
        <tr><th>File</th><th>Test</th><th>Coverage</th></tr>
    </thead>
    <tbody>
        <tr>
            <td rowspan="3"><a href="crates/typestate-pipeline/tests/safety/factory_no_leak.rs"><code>factory_no_leak.rs</code></a></td>
            <td><code>fallible_setter_failure_drops_other_set_fields</code></td>
            <td>Failing fallible setter still drops the previously-set fields (no leak via <code>ManuallyDrop</code>).</td>
        </tr>
        <tr>
            <td><code>fallible_overrider_failure_drops_other_set_fields</code></td>
            <td>Failing fallible overrider drops both the new and the unrelated set fields.</td>
        </tr>
        <tr>
            <td><code>async_setter_dropped_mid_await_drops_other_set_fields</code></td>
            <td>Async setter's future dropped mid-<code>await</code> still drops the other set fields.</td>
        </tr>
        <tr>
            <td rowspan="4"><a href="crates/typestate-pipeline/tests/safety/factory_panic_safety.rs"><code>factory_panic_safety.rs</code></a></td>
            <td><code>panic_in_drop_still_drops_subsequent_fields</code></td>
            <td>The bag's manual <code>Drop</code> reads each set field into its own <code>Option&lt;T&gt;</code> stack guard, so a panic in field N's <code>T::drop</code> still drops fields N+1..end via auto-drop's cleanup-on-panic.</td>
        </tr>
        <tr>
            <td><code>panic_in_default_expr_during_finalize_drops_already_read_fields</code></td>
            <td><code>finalize()</code> reads all initialized fields into stack locals before evaluating any <code>default = …</code> expression, so a panicking default unwinds with every already-read field as an owned local that auto-drops.</td>
        </tr>
        <tr>
            <td><code>panic_in_old_value_drop_during_override_drops_other_fields</code></td>
            <td><code>override_&lt;field&gt;()</code> reads the OLD value into a stack temp before constructing the new bag; a panicking <code>T::drop</code> on the old value unwinds with the new bag in scope, and its panic-safe <code>Drop</code> reclaims the other fields.</td>
        </tr>
        <tr>
            <td><code>panic_in_old_value_drop_during_remove_drops_other_fields</code></td>
            <td>Same shape for <code>drop_&lt;field&gt;()</code>: removed value drops at end-of-scope, after the new bag has been constructed.</td>
        </tr>
        <tr>
            <td rowspan="3"><a href="crates/typestate-pipeline/tests/safety/factory_hygiene.rs"><code>factory_hygiene.rs</code></a></td>
            <td><code>struct_with_field_names_matching_macro_internals_compiles</code></td>
            <td>A struct with fields named <code>_markers</code>, <code>this</code>, <code>__field_value</code>, <code>__old_field</code>, and <code>__new_bag</code> compiles cleanly — the macro's internals are prefixed with <code>__tsh_</code> so they cannot collide with user-declared field names.</td>
        </tr>
        <tr>
            <td><code>override_and_drop_field_named_old_field_round_trips</code></td>
            <td><code>override_&lt;field&gt;</code> and <code>drop_&lt;field&gt;</code> still work for fields whose names previously collided with the macro's stack temps.</td>
        </tr>
        <tr>
            <td><code>default_expression_can_call_user_scope_function</code></td>
            <td><code>default = user_helper()</code> resolves a free function in the user's scope; the rename did not over-isolate the default expression's environment.</td>
        </tr>
        <tr>
            <td rowspan="4"><a href="crates/typestate-pipeline/tests/safety/factory_phantom_shape.rs"><code>factory_phantom_shape.rs</code></a></td>
            <td><code>all_internal_struct_finalizes_without_flag_generics</code></td>
            <td>Bag with zero flag generics (every field is <code>internal</code>) compiles to <code>PhantomData&lt;()&gt;</code> and round-trips through <code>finalize</code>.</td>
        </tr>
        <tr>
            <td><code>singleton_flag_struct_round_trips</code></td>
            <td>One-flag bag compiles to <code>PhantomData&lt;(F,)&gt;</code> (singleton tuple) and round-trips. The trailing comma in the macro template is load-bearing — without it, the marker would be <code>PhantomData&lt;F&gt;</code> and silently change variance.</td>
        </tr>
        <tr>
            <td><code>one_flag_bag_is_send_and_sync_when_field_is</code></td>
            <td>Auto-trait inheritance through <code>PhantomData&lt;(F,)&gt;</code> matches the many-flag case — <code>Send</code> / <code>Sync</code> are forwarded through the tuple, not collapsed to a bare <code>F</code>.</td>
        </tr>
        <tr>
            <td><code>many_flag_struct_round_trips</code></td>
            <td>Three-flag bag (control case) compiles and round-trips through <code>finalize</code>.</td>
        </tr>
        <tr>
            <td><a href="crates/typestate-pipeline/tests/ui/pipeline_field_is_private.rs"><code>ui/pipeline_field_is_private.rs</code></a></td>
            <td><code>ui_compile_failures</code></td>
            <td>Compile-fail trybuild case: reading <code>pipeline.ctx</code> / <code>pipeline.inner</code> directly is rejected by the privacy check. The diagnostic suggests calling <code>ctx()</code> / <code>into_parts()</code> instead.</td>
        </tr>
        <tr>
            <td rowspan="22"><a href="crates/typestate-pipeline/tests/safety/factory_no_unsafe.rs"><code>factory_no_unsafe.rs</code></a></td>
            <td><code>build_in_order</code></td>
            <td><code>no_unsafe</code> path: setters in declared order; <code>finalize()</code> returns the populated struct.</td>
        </tr>
        <tr>
            <td><code>build_in_arbitrary_order</code></td>
            <td><code>no_unsafe</code> path: out-of-order setters still typecheck and finalize.</td>
        </tr>
        <tr>
            <td><code>default_helper_fills_in_optional</code></td>
            <td><code>no_unsafe</code> path: <code>&lt;field&gt;_default()</code> flips an optional's flag to <code>Yes</code>.</td>
        </tr>
        <tr>
            <td><code>finalize_uses_default_when_optional_unset</code></td>
            <td><code>no_unsafe</code> path: optional resolves via <code>Storage::finalize_or</code> at monomorphization, not a runtime branch.</td>
        </tr>
        <tr>
            <td><code>getter_borrows_set_field</code></td>
            <td><code>no_unsafe</code> path: getter returns <code>&amp;T</code> from the <code>Storage&lt;T&gt;</code> sister-shape once flag = <code>Yes</code>.</td>
        </tr>
        <tr>
            <td><code>empty_bag_dropped_does_not_touch_unset_fields</code></td>
            <td><code>no_unsafe</code> path: empty bag drops cleanly via auto-derived <code>Drop</code> on <code>()</code> slots.</td>
        </tr>
        <tr>
            <td><code>partial_bag_dropped_drops_only_set_fields</code></td>
            <td><code>no_unsafe</code> path: partial bag's auto-Drop runs on <code>T</code> slots only.</td>
        </tr>
        <tr>
            <td><code>fully_populated_bag_dropped_drops_all</code></td>
            <td><code>no_unsafe</code> path: fully-set bag's auto-Drop runs every field's destructor.</td>
        </tr>
        <tr>
            <td><code>finalize_does_not_double_drop</code></td>
            <td><code>no_unsafe</code> path: <code>finalize()</code> moves fields out without re-running <code>Drop</code>.</td>
        </tr>
        <tr>
            <td><code>drop_field_drops_value_once</code></td>
            <td><code>no_unsafe</code> path: <code>drop_&lt;field&gt;()</code> swaps the slot to <code>()</code> and runs <code>Drop</code> exactly once.</td>
        </tr>
        <tr>
            <td><code>drop_field_then_reset_doesnt_double_drop</code></td>
            <td><code>no_unsafe</code> path: re-setting a removed field doesn't double-drop the original value.</td>
        </tr>
        <tr>
            <td><code>override_drops_old_value</code></td>
            <td><code>no_unsafe</code> path: <code>override_&lt;field&gt;()</code> drops the prior <code>T</code> before storing the replacement.</td>
        </tr>
        <tr>
            <td><code>finalize_uses_defaults_when_optional_no</code></td>
            <td><code>no_unsafe</code> path: unset optional pulls its declared default at <code>finalize</code>.</td>
        </tr>
        <tr>
            <td><code>finalize_keeps_explicit_values_when_optional_yes</code></td>
            <td><code>no_unsafe</code> path: explicitly-set optional keeps its value at <code>finalize</code>.</td>
        </tr>
        <tr>
            <td><code>finalize_mixes_set_and_default</code></td>
            <td><code>no_unsafe</code> path: mixed explicit + default optionals all resolve correctly.</td>
        </tr>
        <tr>
            <td><code>custom_transformer_fn</code></td>
            <td><code>no_unsafe</code> path: <code>setter = &lt;fn&gt;</code> sync transformer composes with <code>Storage&lt;T&gt;</code>.</td>
        </tr>
        <tr>
            <td><code>fallible_transformer_success</code></td>
            <td><code>no_unsafe</code> path: <code>fallible</code> <code>Ok(_)</code> advances the flag and stores the value.</td>
        </tr>
        <tr>
            <td><code>fallible_transformer_failure</code></td>
            <td><code>no_unsafe</code> path: <code>fallible</code> <code>Err(_)</code> propagates and leaves the bag unchanged.</td>
        </tr>
        <tr>
            <td><code>fallible_setter_failure_drops_other_set_fields</code></td>
            <td><code>no_unsafe</code> path: failing fallible setter still drops the other set fields.</td>
        </tr>
        <tr>
            <td><code>fallible_overrider_failure_drops_other_set_fields</code></td>
            <td><code>no_unsafe</code> path: failing fallible overrider drops both the new and the unrelated values.</td>
        </tr>
        <tr>
            <td><code>async_setter_dropped_mid_await_drops_other_set_fields</code></td>
            <td><code>no_unsafe</code> path: async setter's future dropped mid-<code>await</code> still drops the other set fields.</td>
        </tr>
        <tr>
            <td><code>internal_field_round_trips</code></td>
            <td><code>no_unsafe</code> path: internal field passes through <code>new(…)</code> → <code>finalize()</code> unchanged.</td>
        </tr>
    </tbody>
</table>

### The `no_unsafe` opt-out

Enabling the `no_unsafe` Cargo feature allows individual derives to opt
into a safe codegen path with `#[factory(no_unsafe)]`:

```toml
[dependencies]
typestate-pipeline = { version = "0.1", features = ["no_unsafe"] }
```

```rust
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

## Testing

Tests are grouped by domain — each `tests/<domain>.rs` is the integration
binary, and the actual cases live as child modules under `tests/<domain>/`.

### `factory` — `#[derive(TypestateFactory)]` feature coverage

<table>
    <thead>
        <tr><th>File</th><th>Test</th><th>Coverage</th></tr>
    </thead>
    <tbody>
        <tr>
            <td rowspan="19"><a href="crates/typestate-pipeline/tests/factory/core.rs"><code>core.rs</code></a></td>
            <td><code>build_in_order</code></td>
            <td>Setters called in declared order; <code>finalize()</code> returns the populated struct.</td>
        </tr>
        <tr>
            <td><code>build_in_arbitrary_order</code></td>
            <td>Setters called out of declared order still typecheck and finalize.</td>
        </tr>
        <tr>
            <td><code>default_helper_fills_in_optional</code></td>
            <td>Generated <code>&lt;field&gt;_default()</code> flips an optional-with-default field's flag to <code>Yes</code>.</td>
        </tr>
        <tr>
            <td><code>getter_borrows_set_field</code></td>
            <td>Getter returns <code>&amp;T</code> once the field's flag is <code>Yes</code>.</td>
        </tr>
        <tr>
            <td><code>empty_bag_dropped_does_not_touch_uninit_fields</code></td>
            <td>Dropping a fresh bag skips every <code>MaybeUninit</code> slot.</td>
        </tr>
        <tr>
            <td><code>partial_bag_dropped_drops_only_set_fields</code></td>
            <td>Drop runs on <code>Yes</code>-flagged fields only, leaving <code>No</code> slots untouched.</td>
        </tr>
        <tr>
            <td><code>fully_populated_bag_dropped_drops_all</code></td>
            <td>Dropping a fully-set bag runs every field's destructor.</td>
        </tr>
        <tr>
            <td><code>finalize_does_not_double_drop</code></td>
            <td><code>finalize()</code> moves fields out without re-running <code>Drop</code> on the bag.</td>
        </tr>
        <tr>
            <td><code>drop_field_drops_value_once</code></td>
            <td><code>drop_&lt;field&gt;()</code> runs the field's destructor exactly once and flips <code>Yes</code> → <code>No</code>.</td>
        </tr>
        <tr>
            <td><code>drop_field_then_reset_doesnt_double_drop</code></td>
            <td>Re-setting a previously-dropped field does not double-drop the original value.</td>
        </tr>
        <tr>
            <td><code>override_drops_old_value</code></td>
            <td><code>override_&lt;field&gt;()</code> drops the previous value before installing the new one.</td>
        </tr>
        <tr>
            <td><code>finalize_uses_defaults_when_optional_no</code></td>
            <td>Unset optional pulls its declared default expression at <code>finalize</code>.</td>
        </tr>
        <tr>
            <td><code>finalize_keeps_explicit_values_when_optional_yes</code></td>
            <td>Explicitly-set optional keeps its value at <code>finalize</code> (default not consulted).</td>
        </tr>
        <tr>
            <td><code>finalize_mixes_set_and_default</code></td>
            <td><code>finalize</code> mixes explicit and default values across multiple optionals.</td>
        </tr>
        <tr>
            <td><code>custom_bag_name</code></td>
            <td><code>#[factory(name = …)]</code> renames the generated bag type.</td>
        </tr>
        <tr>
            <td><code>custom_setter_name</code></td>
            <td><code>#[field(setter = &lt;ident&gt;)]</code> renames the setter method.</td>
        </tr>
        <tr>
            <td><code>custom_transformer_fn</code></td>
            <td><code>setter = &lt;fn&gt;</code> runs a sync transformer between user input and the stored field.</td>
        </tr>
        <tr>
            <td><code>fallible_transformer_success</code></td>
            <td><code>fallible</code> transformer's <code>Ok(_)</code> advances the flag and stores the unwrapped value.</td>
        </tr>
        <tr>
            <td><code>fallible_transformer_failure</code></td>
            <td><code>fallible</code> transformer's <code>Err(_)</code> propagates and leaves the bag unchanged.</td>
        </tr>
        <tr>
            <td rowspan="6"><a href="crates/typestate-pipeline/tests/factory/async.rs"><code>async.rs</code></a></td>
            <td><code>standalone_async_setter_non_fallible</code></td>
            <td><code>async_fn</code> setter advances the flag once awaited.</td>
        </tr>
        <tr>
            <td><code>standalone_async_setter_fallible_failure</code></td>
            <td>Async fallible setter's <code>Err</code> after <code>.await</code> leaves the bag unchanged.</td>
        </tr>
        <tr>
            <td><code>standalone_async_finalize</code></td>
            <td><code>finalize_async(via = …, into = …)</code> awaits the converter into the target type.</td>
        </tr>
        <tr>
            <td><code>pipeline_async_setter_chains_through_inflight</code></td>
            <td>Async setter on the carrier arm threads through <code>InFlight</code> without breaking the chain.</td>
        </tr>
        <tr>
            <td><code>pipeline_async_fallible_setter_propagates_error</code></td>
            <td>Async fallible setter on the carrier surfaces <code>Err</code> at the terminal <code>.await?</code>.</td>
        </tr>
        <tr>
            <td><code>transitions_body_calls_finalize</code></td>
            <td><code>#[transitions]</code> body can call <code>.finalize()</code> on an async bag mid-chain.</td>
        </tr>
        <tr>
            <td rowspan="3"><a href="crates/typestate-pipeline/tests/factory/input_type.rs"><code>input_type.rs</code></a></td>
            <td><code>setter_takes_input_type_not_field_type</code></td>
            <td>Setter accepts the <code>input = …</code> type; transformer maps it into the storage type.</td>
        </tr>
        <tr>
            <td><code>default_helper_bypasses_transformer</code></td>
            <td>Default helper writes the storage type directly, skipping the input-type transformer.</td>
        </tr>
        <tr>
            <td><code>unset_default_field_uses_default_at_finalize</code></td>
            <td>An unset <code>default</code> field with <code>input = …</code> still finalizes via the default expression.</td>
        </tr>
        <tr>
            <td rowspan="7"><a href="crates/typestate-pipeline/tests/factory/internal_field.rs"><code>internal_field.rs</code></a></td>
            <td><code>constructor_takes_internal_field_as_argument</code></td>
            <td><code>new(…)</code> takes internal fields positionally — no setter chain needed.</td>
        </tr>
        <tr>
            <td><code>internal_getter_is_unconditional</code></td>
            <td>Internal-field getter is callable on every bag state (no <code>Yes</code> flag bound).</td>
        </tr>
        <tr>
            <td><code>internal_field_dropped_from_flag_generic_list</code></td>
            <td>Bag's flag-generic list omits internals — type signature has one fewer parameter than fields.</td>
        </tr>
        <tr>
            <td><code>pipeline_arm_works_for_non_internal_fields</code></td>
            <td>Carrier setters generated for non-internal fields still drive the bag correctly.</td>
        </tr>
        <tr>
            <td><code>finalize_passes_internal_field_through</code></td>
            <td><code>finalize()</code> reads the internal value from plain-<code>T</code> storage into the constructed struct.</td>
        </tr>
        <tr>
            <td><code>carrier_internal_getter_is_unconditional</code></td>
            <td>Carrier-arm internal getter has no flag bound — callable on any Resolved-mode carrier.</td>
        </tr>
        <tr>
            <td><code>carrier_non_internal_getter_gates_on_yes_flag</code></td>
            <td>Carrier-arm non-internal getter requires the field's flag = <code>Yes</code> to compile.</td>
        </tr>
        <tr>
            <td rowspan="3"><a href="crates/typestate-pipeline/tests/factory/ready_trait.rs"><code>ready_trait.rs</code></a></td>
            <td><code>ready_trait_is_implemented_when_required_flags_yes</code></td>
            <td><code>&lt;Bag&gt;Ready</code> is implemented as soon as every required flag is <code>Yes</code>.</td>
        </tr>
        <tr>
            <td><code>ready_trait_works_when_optional_set_too</code></td>
            <td>Trait still applies when optional fields are explicitly set (not just defaulted).</td>
        </tr>
        <tr>
            <td><code>dispatch_via_trait_matches_inherent_finalize</code></td>
            <td>Generic <code>B: &lt;Bag&gt;Ready</code> dispatch returns the same value as inherent <code>finalize()</code>.</td>
        </tr>
    </tbody>
</table>

### `transitions` — `#[transitions]` body shapes and `Pipelined` resolution

<table>
    <thead>
        <tr><th>File</th><th>Test</th><th>Coverage</th></tr>
    </thead>
    <tbody>
        <tr>
            <td rowspan="5"><a href="crates/typestate-pipeline/tests/transitions/core.rs"><code>core.rs</code></a></td>
            <td><code>full_chain_with_resolved_breakpoint_in_middle</code></td>
            <td>Resolved → InFlight → Resolved chain folds into a single terminal <code>.await?</code>.</td>
        </tr>
        <tr>
            <td><code>breakpoint_forces_explicit_await</code></td>
            <td><code>deferred = false</code> on an async transition forces an explicit <code>.await</code> in the chain.</td>
        </tr>
        <tr>
            <td><code>sync_fallible_resolved_returns_result_directly</code></td>
            <td>Sync fallible Resolved transition returns <code>Result&lt;_, _&gt;</code> without wrapping in a future.</td>
        </tr>
        <tr>
            <td><code>sync_fallible_propagates_through_inflight_chain</code></td>
            <td>Sync fallible step inside an InFlight chain surfaces its <code>Err</code> at the terminal <code>.await?</code>.</td>
        </tr>
        <tr>
            <td><code>intofuture_resolves_inflight_back_to_resolved</code></td>
            <td><code>.await</code> on an InFlight terminal yields the next Resolved value.</td>
        </tr>
        <tr>
            <td rowspan="4"><a href="crates/typestate-pipeline/tests/transitions/via_pipelined.rs"><code>via_pipelined.rs</code></a></td>
            <td><code>transitions_chain_without_error_arg</code></td>
            <td><code>pipelined!(…)</code> declared without an <code>error =</code> arg still composes a transition chain.</td>
        </tr>
        <tr>
            <td><code>transitions_chain_propagates_error</code></td>
            <td>With <code>error = …</code>, errors propagate through the GAT-resolved chain.</td>
        </tr>
        <tr>
            <td><code>factory_pipeline_arms_without_error_arg</code></td>
            <td>Factory carriers without an error type still get the generated pipeline arm.</td>
        </tr>
        <tr>
            <td><code>intofuture_provided_by_pipelined</code></td>
            <td><code>Pipelined</code> impl supplies the <code>IntoFuture</code> that drives InFlight back to Resolved.</td>
        </tr>
        <tr>
            <td><a href="crates/typestate-pipeline/tests/transitions/attr_forwarding.rs"><code>attr_forwarding.rs</code></a></td>
            <td><code>impl_attr_forwarded_to_both_arms</code></td>
            <td>Attributes on the source <code>impl</code> are forwarded onto both generated Resolved + InFlight arms.</td>
        </tr>
    </tbody>
</table>

### `integration` — cross-feature scenarios (factory + pipeline + transitions)

<table>
    <thead>
        <tr><th>File</th><th>Test</th><th>Coverage</th></tr>
    </thead>
    <tbody>
        <tr>
            <td rowspan="4"><a href="crates/typestate-pipeline/tests/integration/factory_pipeline.rs"><code>factory_pipeline.rs</code></a></td>
            <td><code>pipeline_setters_chain_in_resolved_mode</code></td>
            <td>Bag setters reach the carrier in Resolved mode and finalize on the carrier side.</td>
        </tr>
        <tr>
            <td><code>pipeline_setters_chain_through_inflight</code></td>
            <td>Bag setters thread through the carrier's InFlight arm without breaking the chain.</td>
        </tr>
        <tr>
            <td><code>pipeline_drop_field_transitions_yes_to_no</code></td>
            <td><code>drop_&lt;field&gt;</code> on the carrier flips the carrier-side flag <code>Yes</code> → <code>No</code>.</td>
        </tr>
        <tr>
            <td><code>pipeline_override_replaces_value</code></td>
            <td><code>override_&lt;field&gt;</code> on the carrier replaces the bag value via the carrier arm.</td>
        </tr>
        <tr>
            <td rowspan="2"><a href="crates/typestate-pipeline/tests/integration/factory_in_pipeline.rs"><code>factory_in_pipeline.rs</code></a></td>
            <td><code>full_chain_bag_into_pipeline</code></td>
            <td>Standalone factory bag's <code>finalize()</code> feeds a pipeline transition mid-chain.</td>
        </tr>
        <tr>
            <td><code>validation_failure_at_bag_finalize</code></td>
            <td>Bag-level fallible finalize short-circuits the surrounding pipeline chain with its <code>Err</code>.</td>
        </tr>
        <tr>
            <td rowspan="5"><a href="crates/typestate-pipeline/tests/integration/dataset_authoring.rs"><code>dataset_authoring.rs</code></a></td>
            <td><code>new_evm_rpc_flow_terminates_at_deployed</code></td>
            <td>EVM-RPC dataset flow chains through every phase to <code>Deployed</code>. (Gated on <code>dataset-authoring-example</code>.)</td>
        </tr>
        <tr>
            <td><code>new_derived_flow_chains_through_single_await</code></td>
            <td>Derived-dataset flow folds into a single terminal <code>.await?</code>.</td>
        </tr>
        <tr>
            <td><code>bump_patch_increments_existing_version</code></td>
            <td>Patch-bump flow advances an existing version's number arithmetically.</td>
        </tr>
        <tr>
            <td><code>bump_patch_errors_when_no_prior_version</code></td>
            <td>Patch-bump fails cleanly when no prior version exists.</td>
        </tr>
        <tr>
            <td><code>edit_existing_kind_mismatch_surfaces_error</code></td>
            <td>Editing a dataset of the wrong kind surfaces a typed error.</td>
        </tr>
    </tbody>
</table>

### `pipeline` — dual-mode `Pipeline` carrier and `inspect` combinator

<table>
    <thead>
        <tr><th>File</th><th>Test</th><th>Coverage</th></tr>
    </thead>
    <tbody>
        <tr>
            <td rowspan="4"><a href="crates/typestate-pipeline/tests/pipeline/impl_pipelined.rs"><code>impl_pipelined.rs</code></a></td>
            <td><code>pipelined_associated_types_resolve</code></td>
            <td><code>Pipelined</code> associated types compile for the canonical carrier shape.</td>
        </tr>
        <tr>
            <td><code>gat_projections_are_correct</code></td>
            <td><code>Resolved&lt;S&gt;</code> / <code>InFlight&lt;S&gt;</code> GAT projections produce the expected concrete carrier types.</td>
        </tr>
        <tr>
            <td><code>intofuture_drives_inflight_back_to_resolved</code></td>
            <td><code>IntoFuture</code> on InFlight awaits to a Resolved successor.</td>
        </tr>
        <tr>
            <td><code>tagged_pipelined_resolves</code></td>
            <td>A carrier with extra generics still satisfies <code>Pipelined</code>.</td>
        </tr>
        <tr>
            <td rowspan="4"><a href="crates/typestate-pipeline/tests/pipeline/inspect.rs"><code>inspect.rs</code></a></td>
            <td><code>resolved_inspect_runs_sync_and_preserves_chain</code></td>
            <td><code>inspect(…)</code> on Resolved runs synchronously and threads the carrier through unchanged.</td>
        </tr>
        <tr>
            <td><code>resolved_inspect_does_not_change_typestate</code></td>
            <td><code>inspect</code> returns <code>Self</code>, so the state generic is unchanged downstream.</td>
        </tr>
        <tr>
            <td><code>inflight_inspect_runs_after_pending_resolves</code></td>
            <td><code>inspect</code> on InFlight runs only after the pending future resolves.</td>
        </tr>
        <tr>
            <td><code>inflight_inspect_chains_through_subsequent_transitions</code></td>
            <td><code>inspect</code> on InFlight returns InFlight, so subsequent transitions still chain.</td>
        </tr>
    </tbody>
</table>

### `ui` — trybuild compile-fail diagnostics

<table>
    <thead>
        <tr><th>File</th><th>Test</th><th>Coverage</th></tr>
    </thead>
    <tbody>
        <tr>
            <td><a href="crates/typestate-pipeline/tests/ui_macros.rs"><code>ui_macros.rs</code></a></td>
            <td><code>ui_compile_failures</code></td>
            <td>Drives trybuild over <a href="crates/typestate-pipeline/tests/ui/"><code>tests/ui/*.rs</code></a> — pins the wording of macro diagnostics for misuse (wrong attrs, unset required fields, etc.).</td>
        </tr>
    </tbody>
</table>

## License

Licensed under either of

- Apache License, Version 2.0
- MIT License

at your option.
