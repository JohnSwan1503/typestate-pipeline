//! Proc-macros for the [`typestate-pipeline`] ecosystem.
//!
//! Two macros, two axes of typestate:
//!
//! - [`transitions`] — attribute on an `impl` block. Each method marked
//!   `#[transition(into = NextState)]` expands into a Resolved + InFlight
//!   method pair on a [`Pipeline`] newtype carrier.
//! - [`TypestateFactory`] — derive on a struct. Generates a sibling
//!   `<Name>Factory<F1, …>` with one flag generic per field; `finalize()`
//!   is callable only once the required flags reach `Yes`.
//!
//! Use the macros through the [`typestate-pipeline`] facade — generated code
//! references items via `::typestate_pipeline::__private::*` and depending
//! on this crate alone produces unresolved paths.
//!
//! [`Pipeline`]: https://docs.rs/typestate-pipeline-core
//! [`typestate-pipeline`]: https://docs.rs/typestate-pipeline

use proc_macro::TokenStream;
use syn::{DeriveInput, ItemImpl, parse_macro_input};

mod diag;
mod prefix;
mod transitions;
mod typestate_factory;

/// Generate Resolved + InFlight method pairs from a single source body.
///
/// Decorates an inherent `impl` block on a tuple-struct newtype around
/// `Pipeline`. Each method marked `#[transition(into = NextState)]`
/// expands into two methods: one for `Resolved` mode and one for
/// `InFlight` mode, both forwarding to a shared private body fn.
///
/// # Arguments
///
/// - `error = <Type>` *(optional)* — error type used by fallible bodies
///   and the `Pipeline`'s `E` parameter. When omitted, the error is read
///   from `<Self as Pipelined<'a>>::Error`.
/// - `ctx = <ident>` *(optional, default `ctx`)* — name the macro
///   recognizes as the borrowed-context parameter inside transition bodies.
///
/// # Method contract
///
/// Each `#[transition]` method is written as if the body owned the state:
///
/// - First parameter must be `state: <CurrentState>`.
/// - Second parameter, if named `ctx` (configurable), receives the
///   borrowed pipeline context.
/// - Remaining parameters become the user-visible parameters on the
///   generated methods (which take `self` as their receiver).
///
/// # Body shapes
///
/// The macro inspects each method's signature and picks one of four
/// expansions:
///
/// | shape            | when                                              | resolved arm                      | in-flight arm                                      |
/// |------------------|---------------------------------------------------|-----------------------------------|----------------------------------------------------|
/// | sync infallible  | `fn` returning a non-`Result`                     | `Carrier<…, Resolved>`            | `Carrier<…, InFlight>`                             |
/// | sync fallible    | `fn` returning `Result<_, E>`                     | `Result<Carrier<…, Resolved>, E>` | `Carrier<…, InFlight>` (Result folded into future) |
/// | async deferred   | `async fn` (default)                              | `Carrier<…, InFlight>`            | `Carrier<…, InFlight>`                             |
/// | async breakpoint | `async fn` with `#[transition(breakpoint)]`       | `async fn` returning `Result<Carrier<…, Resolved>, E>` | same                          |
///
/// **Deferred async** (the default) lets a chain like
/// `pipeline.tag(7).with_parallelism(8).deploy().await?` fold every async
/// transition into a single terminal `.await?`. **Breakpoint async**
/// resolves at that method, useful when a later transition needs the
/// resolved state value to compute its arguments.
///
/// # Example
///
/// ```ignore
/// use typestate_pipeline::{pipelined, transitions};
///
/// pipelined!(Author, ctx = Client, error = AuthoringError);
///
/// #[transitions]
/// impl<'a> Author<'a, Registered> {
///     #[transition(into = Versioned)]
///     pub async fn tag_version(state: Registered, ctx: &Client, version: u32)
///         -> Result<Versioned, AuthoringError>
///     {
///         ctx.tag(state.name.clone(), version).await;
///         Ok(Versioned { name: state.name, version })
///     }
/// }
/// ```
///
/// # Safety
///
/// Generated code uses no `unsafe`. Every shape expands into safe glue
/// around `Pipeline::resolved`, `Pipeline::in_flight`, `Box::pin`, and
/// `Pipeline::map_inner_sync` / `map_inner_sync_fallible`. State-machine
/// soundness is enforced by the type system: a transition wired to the
/// wrong destination state simply will not compile at the call site.
#[doc(alias = "state machine")]
#[doc(alias = "typestate")]
#[doc(alias = "transition")]
#[proc_macro_attribute]
pub fn transitions(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as transitions::TransitionsArgs);
    let input = parse_macro_input!(item as ItemImpl);
    match transitions::expand(args, input) {
        Ok(ts) => ts.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

/// Derive a sibling typestate factory for a struct.
///
/// Generates `<Name>Factory<F1, …>`, one flag generic per field. Setters
/// transition the relevant flag from `No` to `Yes`; `finalize()` is
/// callable only when every required flag is `Yes`.
///
/// # Container attributes — `#[factory(...)]`
///
/// - `name = MyFactory` — override the generated factory's type name
///   (default `<Original>Factory`).
/// - `error = MyError` — error type for fallible setters and Pipeline arms.
///   Required when any field is `fallible`.
/// - `pipeline(carrier = MyAuthor)` — also emit Resolved + InFlight method
///   pairs on the user's carrier for every standalone transition. The
///   carrier must implement `Pipelined<'a>`.
/// - `finalize_async(via = my_fn, into = Target, error = E?)` — emit an
///   additional `async fn finalize_async()` that calls `my_fn(raw).await`.
/// - `no_unsafe` — opt into the safe-mode codegen path. Gated on the
///   `no_unsafe` Cargo feature; without the feature the attribute is
///   rejected at expansion time.
///
/// # Field attributes — `#[field(...)]`
///
/// - `required` *(default unless `default = …` is present)* — flag must
///   be `Yes` for `finalize`.
/// - `optional` — opt out of required.
/// - `default` / `default = expr` — declare a default; emits a
///   `<field>_default()` helper. Optional fields with defaults may
///   finalize whether the flag is `Yes` or `No`.
/// - `removable` — emit `drop_<field>(self)` reverting the flag to `No`.
/// - `overridable` — emit `override_<field>(self, val)` on `Yes`-flagged
///   bags.
/// - `name = setter_name` — override the generated setter's name.
/// - `setter = my_fn` — call `my_fn(val) -> FieldType` inside the setter.
/// - `fallible` *(with `setter`)* — transformer returns
///   `Result<FieldType, Error>`; setter returns `Result<NextBag, Error>`.
/// - `async_fn` *(with `setter`)* — transformer is `async fn`; setter
///   becomes `async fn`.
/// - `internal` — locked at construction; field is set positionally on
///   `new(…)` and has no setter / overrider / remover / default.
/// - `input = <Type>` *(with `setter`)* — override the setter's input
///   parameter type; the transformer bridges to the field's storage type.
///
/// `default` is rejected when combined with `fallible` or `async_fn`.
///
/// # Example
///
/// ```ignore
/// use typestate_pipeline::TypestateFactory;
///
/// #[derive(TypestateFactory)]
/// struct User {
///     #[field(required)]
///     name: String,
///     #[field(required)]
///     email: String,
///     #[field(default = 18)]
///     age: u32,
/// }
///
/// let user = UserFactory::new()
///     .name("Alice".into())
///     .email("alice@example.com".into())
///     .with_age(30)
///     .finalize();
/// ```
///
/// # Safety
///
/// By default the generated code uses three `unsafe` operations, each
/// gated by a type-level invariant:
///
/// 1. `MaybeUninit::assume_init_ref` in getters — gated by the field's
///    flag being pinned to `Yes` in the impl signature.
/// 2. `MaybeUninit::assume_init_drop` in `Drop` — guarded at runtime by
///    the flag's `Satisfiable::IS_SET` constant.
/// 3. `ptr::read` in setters / `finalize` — paired with `ManuallyDrop`
///    so the moved-from `MaybeUninit` slot is not touched again.
///
/// Setters compute the new value *before* wrapping `self` in
/// `ManuallyDrop`, so a failing `?` or dropped future leaves `self` live
/// and its normal `Drop` runs. The `factory_no_leak` test suite is the
/// regression guard.
///
/// Opting in to `#[factory(no_unsafe)]` (with the `no_unsafe` Cargo
/// feature on) emits a `MaybeUninit`-free implementation that uses
/// `<Flag as Storage<T>>::Out` to give each flag combination a
/// structurally distinct type. See [`Storage`] for details.
///
/// [`Storage`]: https://docs.rs/typestate-pipeline-core/latest/typestate_pipeline_core/flag/trait.Storage.html
#[doc(alias = "builder")]
#[doc(alias = "typestate")]
#[doc(alias = "accumulator")]
#[proc_macro_derive(TypestateFactory, attributes(factory, field))]
pub fn derive_typestate_factory(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match typestate_factory::expand(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}
