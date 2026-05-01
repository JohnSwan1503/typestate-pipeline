# typestate-pipeline

[![crates.io](https://img.shields.io/crates/v/typestate-pipeline.svg)](https://crates.io/crates/typestate-pipeline)
[![docs.rs](https://img.shields.io/docsrs/typestate-pipeline)](https://docs.rs/typestate-pipeline)
[![CI](https://github.com/JohnSwan1503/typestate-pipeline/actions/workflows/ci.yml/badge.svg)](https://github.com/JohnSwan1503/typestate-pipeline/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/crates/msrv/typestate-pipeline)](https://github.com/JohnSwan1503/typestate-pipeline/blob/main/Cargo.toml)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/typestate-pipeline.svg)](#license)

Compile-time-checked typestate scaffolding for Rust: a dual-mode
pipeline carrier for cross-phase state machines, and a named-field
accumulator derive for argument bags. The two macros compose — a
factory can run inside a pipeline phase, with its setters landing
directly on the user's carrier.

> **Looking for a feature?** Every macro option has a worked recipe in
> [`recipes`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/recipes/)
> on docs.rs — source code paired with a sketch of what the macro emits,
> backed by the [`tests/expansions`](https://github.com/JohnSwan1503/typestate-pipeline/tree/main/crates/typestate-pipeline/tests/expansions/)
> suite that locks the surface in.

```rust
use typestate_pipeline::{Pipeline, TypestateFactory, pipelined, transitions};

#[derive(TypestateFactory)]
struct Profile {
    #[field(required)]   name: String,
    #[field(required)]   email: String,
    #[field(default = 18)] age: u32,
}

#[derive(Debug)]
struct AuthError(&'static str);
impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(self.0) }
}
impl std::error::Error for AuthError {}

pipelined!(Author, ctx = (), error = AuthError);

struct Registered { profile: Profile }
struct Deployed   { profile: Profile, account_id: u64 }

#[transitions]
impl<'a> Author<'a, Registered> {
    #[transition(into = Deployed)]
    pub async fn deploy(state: Registered) -> Result<Deployed, AuthError> {
        Ok(Deployed { profile: state.profile, account_id: 42 })
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), AuthError> {
    let profile = ProfileFactory::new()
        .name("Alice".into())
        .email("alice@example.com".into())
        .with_age(30)            // optional — overrides the default of 18
        .finalize();

    let deployed = Author(Pipeline::resolved(&(), Registered { profile }))
        .deploy()
        .await?;

    let state = deployed.0.into_state();
    println!("{} got account #{}", state.profile.name, state.account_id);
    Ok(())
}
```

This example lives at
[`examples/minimal.rs`](https://github.com/JohnSwan1503/typestate-pipeline/blob/main/crates/typestate-pipeline/examples/minimal.rs)
— run it with `cargo run --example minimal`. For the full feature
surface — `pipeline(carrier = …)` composition, the `<Bag>Ready`
companion trait, async-fallible setters that lift the chain to
`InFlight` — see
[`examples/quickstart.rs`](https://github.com/JohnSwan1503/typestate-pipeline/blob/main/crates/typestate-pipeline/examples/quickstart.rs).
For a multi-phase pipeline with an async-fetch breakpoint, see
[`examples/dataset_authoring.rs`](https://github.com/JohnSwan1503/typestate-pipeline/blob/main/crates/typestate-pipeline/examples/dataset_authoring.rs)
(behind the `dataset-authoring-example` feature).

Try removing `.email(...)` from the chain — the compiler refuses
`.finalize()` because the bag's flag tuple no longer matches the
finalize-callable shape. The same compile-time check guards phase
transitions: `.deploy()` on a carrier whose state isn't `Registered`
simply doesn't typecheck.

> **When this is overkill.** A four-field one-shot builder doesn't need
> the typestate machinery — a struct literal or a thin `impl Default`
> is fine. Reach for `typestate-pipeline` when (a) the order of
> operations across multiple steps needs to be enforced at compile
> time, (b) a single bag accumulates many required and optional
> arguments before being consumed, or (c) you have an async pipeline
> whose intermediate states should not be observable to callers.

## Where this fits

Rust has excellent compile-time builders.
[`bon`](https://docs.rs/bon) and
[`typed-builder`](https://docs.rs/typed-builder) both lift
required-field enforcement into the type system through polished derive
APIs, and [`derive_builder`](https://docs.rs/derive_builder) covers the
runtime-checked end of the same space. For a single struct assembled
and consumed in one place, those crates are the right tool — they have
mature ecosystems, careful documentation, and they solve that problem
directly.

`typestate-pipeline` is built around an adjacent question: what
happens when the work a chain represents is itself a sequence of
phases — register, configure, deploy — each carrying its own bag of
required and optional arguments, and where the whole sequence should
read as one expression rather than a string of let-bindings broken at
every `await` boundary. Three design choices follow:

- **The carrier has two modes.** `Resolved` holds the current state;
  `InFlight` holds a pending future. An async transition lifts a chain
  into `InFlight`, and every subsequent step — sync, async, fallible,
  or any mix — folds into that future. The chain reads as one
  expression and awaits once at the end, rather than breaking at every
  async boundary.

- **The builder and the pipeline share a carrier.** A bag declared
  `#[factory(pipeline(carrier = MyAuthor))]` emits its setters on the
  pipeline carrier in both modes, so a phase that accumulates many
  parameters stays inside the chain — no detour to assemble a separate
  builder expression and hand it back in.

- **Each flag combination is a structurally distinct sister type.**
  An implementation choice: required-field enforcement is solved
  identically in established builder crates through `Option<T>` storage
  with phantom-flag generics, and the unwrap is statically guaranteed
  safe in either approach. Distinct sister types let the auto-generated
  `<Bag>Ready` companion trait express "any finalize-callable bag" as
  a single trait bound, rather than forcing generic code to spell out
  the full flag tuple at every use site.

- **Storage cells are `MaybeUninit<T>`, not `Option<T>`.** A coupled
  choice. With the flag carried at the type level, an `Option`'s
  `is_some` discriminator would be redundant — the type already says
  whether the field is set. `MaybeUninit<T>` removes the discriminator
  and keeps the struct layout uniform across every flag combination,
  at the cost of a small set of `unsafe` operations gated by type-level
  invariants (see [Safety](#safety) for the full accounting). The
  `no_unsafe` Cargo feature swaps `MaybeUninit<T>` for
  `<Flag as Storage<T>>::Out` (`T` when set, `()` when unset) for a
  zero-`unsafe` codegen path that trades uniform layout for the
  sister-shape representation.

These are design choices, not feature deltas. If your work is one
struct built once, or a state machine without per-phase arguments, the
established crates above are the more direct fit —
`typestate-pipeline` exists for the specific case where the two
problems compose.

## Mental model

Two orthogonal axes. Each macro operates on one of them, and they
compose freely.

### Factory/Builder axis — `#[derive(TypestateFactory)]`

Every non-`internal` field on a derived bag carries a flag generic that
is either `No` (unset) or `Yes` (set). The full transition graph,
covering every `#[field(…)]` mutability attribute:

|Operation|Flag transition|Notes|
|---|---|---|
|`Factory::new()` / `Default::default()`|initial|every flag `No`|
|`.field(val)`|`No → Yes`|required field, default naming|
|`.with_field(val)`|`No → Yes`|`optional` or `default` field|
|`.field_default()`|`No → Yes`|uses the declared default expression|
|`.drop_field()`|`Yes → No`|requires `removable`; drops the value|
|`.override_field(val)`|`Yes → Yes`|requires `overridable`; drops old, stores new|
|`.finalize()`|consumes `self`|every required flag must be `Yes`|

A field with `#[field(default)]` or `#[field(default = expr)]` may
finalize whether its flag is `Yes` (the user's value is used) or `No`
(the default expression is evaluated). Required fields without a default
have no such relaxation.

`#[field(internal)]` fields don't appear in the flag-generic list at all
— they're set positionally on `new(…)` and have an unconditional getter.

### Carrier axis — `#[transitions]`, `pipelined!`, `impl_pipelined!`

The pipeline carrier is dual-mode: `Resolved` holds the current state
directly, `InFlight` holds a pending future that will yield the next
state. Each `#[transition]` body shape picks which arrow it takes; the
two arms emitted per transition (one per starting mode) end up in
different places:

|Body shape|Resolved arm returns|InFlight arm returns|
|---|---|---|
|**Sync infallible** — `fn` returning `T`|`Resolved`|`InFlight`|
|**Sync fallible** — `fn` returning `Result<T, E>`|`Result<Resolved, E>` (handle at call site)|`InFlight` (folds into pending future)|
|**Async deferred** — `async fn` (default for async)|`InFlight` (lifts the chain)|`InFlight`|
|**Async breakpoint** — `async fn` + `breakpoint`|`async fn → Result<Resolved, E>`|`async fn → Result<Resolved, E>`|

Crosscutting: any `InFlight` carrier `.await?`s into a `Resolved` of the
same state via the carrier's `IntoFuture` impl.

### Chain folding

A chain that mixes every body shape — sync infallible, sync fallible,
and async deferred — folds into a single terminal `.await?`. The
example below is verbatim from
[`tests/transitions/core/tests/full_chain_with_resolved_breakpoint_in_middle.rs`](https://github.com/JohnSwan1503/typestate-pipeline/blob/main/crates/typestate-pipeline/tests/transitions/core/tests/full_chain_with_resolved_breakpoint_in_middle.rs):

```rust,ignore
let deployed: Author<Deployed> = Author::from_registered(&client, "ds-a", 0xCAFE)
    .tag_version(7)              // async deferred  -> lifts Resolved to InFlight
    .with_parallelism(8)         // sync infallible -> folds into pending
    .validate_and_finalize()     // sync fallible   -> folds Result into pending
    .deploy()                    // async deferred  -> folds into pending
    .await?;                     //                 -> Author<Deployed, Resolved>
```

Adding `#[transition(into = …, breakpoint)]` to one of the steps
(an *async breakpoint*) forces the chain to `.await?` at that step,
landing back in `Resolved` for whatever follows.

## Macros

Two proc-macros and one declarative macro pair, each operating on one
of the axes from the [Mental model](#mental-model). They are independent
— either is useful on its own — but compose: a factory can run *inside*
a pipeline phase, and the `pipeline(carrier = …)` arm even emits its
setters directly on the user's carrier.

### `#[derive(TypestateFactory)]`

Generates `<Name>Factory<F1, F2, …>` with one flag generic per field.
Setters consume `self` and transition the relevant flag from `No` to
`Yes`. `finalize()` is callable only when every required flag is `Yes`.
The headline example above shows the baseline shape; each row below
points at a worked recipe (source + expansion sketch) on docs.rs:

|Recipe|What it adds|
|---|---|
|[Minimal](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/guide/index.html#minimal-every-field-required)|baseline — every field required, no options|
|[`required` / `optional`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/guide/index.html#required-vs-optional-setter-naming)|naming change — `field(val)` vs `with_field(val)`|
|[`default` / `default = expr`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/guide/index.html#defaults-relaxing-the-finalize-bound)|optional with fallback; emits `<field>_default()` helper|
|[`removable`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/guide/index.html#removable-un-setting-a-field)|emit `drop_<field>(self)` reverting the flag to `No`|
|[`overridable`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/guide/index.html#overridable-replacing-a-set-value)|emit `override_<field>(self, val)` on `Yes`-flagged bags|
|[`internal`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/guide/index.html#internal-field-locked-at-construction)|positional on `new(…)`, locked from then on|
|[`setter = my_fn`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/guide/index.html#setter-transformer-in-setter-normalization)|run a transformer inside the setter|
|[`setter = …, fallible`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/guide/index.html#fallible-setter-validating-in-the-setter)|transformer returns `Result<_, E>`; setter does too|
|[`setter = …, async_fn`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/guide/index.html#async-setter-io-in-the-setter)|`async` setter; combine with `fallible` for async fallible|
|[`setter = …, input = T`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/guide/index.html#setter-input-type-when-input--storage)|setter input type differs from the storage type|
|[`name = …` / `setter = ident`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/guide/index.html#renaming-when-the-defaults-clash)|rename the bag and individual setters|
|[`pipeline(carrier = …)`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/guide/index.html#pipeline-arm-setters-on-the-carrier)|also emit Resolved + InFlight method pairs on the carrier|
|[`finalize_async(via = …, into = …)`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/guide/index.html#async-finalize-post-finalize-hooks)|async finalize hook|
|[`<Bag>Ready` companion trait](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/guide/index.html#the-ready-trait-generic-over-finalize-callable)|exit-side bound: accept "any finalize-callable bag" generically|
|`<Bag>Empty` companion alias|entry-side type: alias for the all-`No` flag-tuple shape (e.g. `SettingsEmpty = Settings<No, No, No>`)|
|[`#[factory(no_unsafe)]`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/guide/index.html#safe-mode-codegen-no_unsafe)|safe-mode codegen path (see [Safety](#safety))|

The full attribute reference lives in the
[`#[derive(TypestateFactory)]` rustdoc](https://docs.rs/typestate-pipeline-macros/latest/typestate_pipeline_macros/derive.TypestateFactory.html).

### `#[transitions]`

Decorates an `impl` block on a tuple-struct newtype around `Pipeline`.
Each method marked `#[transition(into = NextState)]` is expanded into a
Resolved + InFlight method pair from a single source body. The
destination type is read off the carrier's `Pipelined<'a>` impl as a
GAT projection (`<Self as Pipelined<'a>>::Resolved<NextState>`), so
carriers with extra generics or unusual ordering keep working as long
as the trait impl is correct.

```rust,ignore
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

|Recipe|Body shape|
|---|---|
|[Sync infallible](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/recipes/transitions/index.html#sync-infallible)|`fn` returning a non-`Result`|
|[Sync fallible](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/recipes/transitions/index.html#sync-fallible)|`fn` returning `Result<_, E>`|
|[Async deferred](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/recipes/transitions/index.html#async-deferred)|`async fn` (default) — lifts the chain to `InFlight`|
|[Async breakpoint](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/recipes/transitions/index.html#async-breakpoint)|`async fn` + `breakpoint` — forces an `.await?`|

Generated transition code uses **no** `unsafe`. Full reference:
[`#[transitions]` rustdoc](https://docs.rs/typestate-pipeline-macros/latest/typestate_pipeline_macros/attr.transitions.html).

### `pipelined!` / `impl_pipelined!`

Declarative shorthand for the conventional carrier shape
(`<'a, S, M = Resolved>` tuple-struct newtype around `Pipeline`):

```rust,ignore
// declares the carrier struct + Pipelined impl + IntoFuture forwarding
typestate_pipeline::pipelined!(pub Author, ctx = Client, error = AuthoringError);

// alternative: hand-write the struct (custom derives, extra generics, …)
// and emit only the trait impls
typestate_pipeline::impl_pipelined!(Author, ctx = Client, error = AuthoringError);
```

Both also emit a chainable `inspect(|carrier| …)` combinator on Resolved
and InFlight. See
[`recipes::pipelined`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/recipes/pipelined/)
and
[`recipes::impl_pipelined`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/recipes/impl_pipelined/)
for worked examples.

## Safety

`#[transitions]` and `pipelined!` / `impl_pipelined!` emit no `unsafe`
in either codegen mode. `#[derive(TypestateFactory)]` uses three
`unsafe` operations by default — and a per-derive opt-out (`#[factory(no_unsafe)]`,
gated on the `no_unsafe` Cargo feature) swaps them for a fully-safe
codegen path; jump to [The `no_unsafe` opt-out](#the-no_unsafe-opt-out)
if that's the only thing you want to know.

The three default-mode `unsafe` operations, each gated by a type-level
invariant:

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

### Implementation invariants

The generated code rests on two groups of invariants — ordering, which
keeps the `unsafe` paths panic- and cancellation-safe, and structural,
which keeps the macro's type-level guarantees and hygiene intact. Each
maps to a regression suite linked at the end.

**Ordering invariants** — panic and cancellation safety in the `unsafe` paths:

- **Setter ordering.** The transformer runs before `self` is wrapped in
  `ManuallyDrop`. A failing transformer (a `?` short-circuit) or a
  future dropped mid-await therefore leaves `self` live, and its normal
  `Drop` releases every set field. Inverting the order would leak.

- **`finalize` ordering.** All field reads land in stack locals before
  any `default = …` expression is evaluated. A panic in a default thunk
  unwinds with the already-read fields as owned locals that auto-drop.
  Inlining reads alongside defaults would leak fields *after* a
  panicking default — their `MaybeUninit` slots would still be sitting
  inside the `ManuallyDrop`-wrapped `this`.

- **`override_<field>` / `drop_<field>` ordering.** The OLD value is
  read into a stack temp *and the new bag is constructed* before the
  temp's auto-drop runs. A panic in the old value's `T::drop` therefore
  unwinds with the new bag already in scope; its panic-safe `Drop`
  reclaims the other fields.

**Structural and hygiene invariants** — the macro's type-level guarantees and codegen hygiene:

- **Hygienic internal bindings.** All macro-emitted identifiers carry a
  `__tsh_` prefix — `__tsh_markers` for the phantom field;
  `__tsh_this`, `__tsh_field_value`, `__tsh_old_field`, `__tsh_new_bag`,
  `__tsh_finalize_<field>`, and `__tsh_guard_<field>` for local
  bindings. The prefix is unlikely to collide with a user-supplied
  field name or with an identifier reachable inside a `default = …`
  expression.

- **Explicit `Send` obligations on async pipeline arms.** The async
  Resolved and InFlight arms carry an explicit `where InputBag: Send + 'a,
  OutputBag: Send + 'a` clause. A non-`Send` user field surfaces the
  diagnostic at the impl block instead of inside
  `Box::pin(async move { … })`.

- **Sealed `Pipeline` fields.** `Pipeline::ctx` and `Pipeline::inner`
  are private; proc-macros destructure carriers through the public
  [`Pipeline::into_parts`] / [`Pipeline::ctx`] accessors. A user's
  carrier newtype therefore cannot bypass the typestate machinery by
  hand-substituting `inner` or forging a `_tag` / `_err` marker.

- **Pinned bag layout.** The generated bag struct is annotated
  `#[repr(Rust)]`. `MaybeUninit<T>` reads via `ptr::read` rely on
  default alignment; a future `#[repr(packed)]` would silently break
  that assumption.

- **`PhantomData` marker tuple is always a tuple.** The marker is
  emitted as `PhantomData<( F1, F2, … )>` with a trailing comma after
  *every* element. With one flag this is the singleton `(F,)` rather
  than the parenthesised type `(F)` (which collapses to `F`); with
  zero flags it is `()`. The parenthesised single-type form would
  silently change variance and auto-trait inheritance.

Each invariant above is locked in by a regression suite, mirrored on
docs.rs at
[`tests::safety`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/tests/safety/):

|Suite|What it locks in|
|---|---|
|[`factory_no_leak`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/tests/safety/factory_no_leak/)|failing fallible setter / overrider / dropped async setter still drops the other set fields|
|[`factory_panic_safety`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/tests/safety/factory_panic_safety/)|the three ordering invariants above survive a panicking `T::drop`|
|[`factory_hygiene`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/tests/safety/factory_hygiene/)|user fields named like macro internals compile cleanly; `default = …` still resolves user-scope helpers|
|[`factory_phantom_shape`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/tests/safety/factory_phantom_shape/)|zero-, one-, and many-flag bags round-trip; the singleton `(F,)` preserves variance and auto traits|
|[`factory_no_unsafe`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/tests/safety/factory_no_unsafe/)|parallel coverage suite for the safe-mode codegen path|

[`Pipeline::into_parts`]: https://docs.rs/typestate-pipeline-core/latest/typestate_pipeline_core/pipeline/struct.Pipeline.html#method.into_parts
[`Pipeline::ctx`]: https://docs.rs/typestate-pipeline-core/latest/typestate_pipeline_core/pipeline/struct.Pipeline.html#method.ctx

### The `no_unsafe` opt-out

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

## Workspace layout

```text
typestate-pipeline          # facade — depend on this
├── typestate-pipeline-core    # runtime: Pipeline, Mode, Pipelined, flag traits
└── typestate-pipeline-macros  # proc-macros (use through the facade)
```

The proc-macros emit fully-qualified paths through
`::typestate_pipeline::__private::*`, so always depend on the facade
crate; depending on the macros crate alone produces unresolved paths.

## Further reading

|Where|What|
|---|---|
|[`recipes`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/recipes/)|every macro option, with source + expansion sketch|
|[`tests`](https://docs.rs/typestate-pipeline/latest/typestate_pipeline/tests/)|the integration-test suite rendered as browsable docs|
|[`#[derive(TypestateFactory)]`](https://docs.rs/typestate-pipeline-macros/latest/typestate_pipeline_macros/derive.TypestateFactory.html)|full attribute reference|
|[`#[transitions]`](https://docs.rs/typestate-pipeline-macros/latest/typestate_pipeline_macros/attr.transitions.html)|full attribute reference|
|[`Pipeline`](https://docs.rs/typestate-pipeline-core/latest/typestate_pipeline_core/pipeline/struct.Pipeline.html)|the runtime carrier|

## License

Licensed under either of

- Apache License, Version 2.0
- MIT License

at your option.
