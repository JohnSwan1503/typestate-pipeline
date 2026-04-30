//! Per-field type-level flag primitives used by the `TypestateFactory` derive.
//!
//! Each field on a derived bag carries a flag generic that is either [`Yes`]
//! (the field has been set) or [`No`] (it has not). [`Satisfiable`] surfaces
//! the flag's state as a `const`, [`Satisfied`] is the sub-trait bound for
//! "must be set", and [`Storage`] selects the bag's per-field storage shape
//! in the safe-mode codegen path.

/// Per-field type-level flag selector.
///
/// Sealed; implemented only by [`Yes`] and [`No`]. The associated constant
/// [`IS_SET`](Self::IS_SET) lets the unsafe-mode bag's `Drop` impl drop only
/// initialized fields.
#[cfg_attr(docsrs, doc(notable_trait))]
pub trait Satisfiable: sealed::Satisfiable {
    /// `true` if the flag indicates the field is set.
    const IS_SET: bool;
}

/// Sub-trait of [`Satisfiable`] implemented only by [`Yes`].
///
/// Use as a bound when generic code requires a flag that witnesses
/// initialization.
#[cfg_attr(docsrs, doc(notable_trait))]
pub trait Satisfied: Satisfiable + sealed::Satisfied {}

/// Per-field type-level storage selector for the safe-mode codegen path
/// (the path opted into via `#[factory(no_unsafe)]`).
///
/// Each [`Satisfiable`] flag chooses the bag's storage shape for its field
/// at the type level: [`Yes`] → the field's own type `T`; [`No`] → `()`.
/// The bag's field is declared as `<Flag as Storage<T>>::Out`, so each
/// `(Yes, …)`/`(No, …)` flag combination is a structurally distinct sister
/// type. This eliminates the need for `MaybeUninit` and a manual `Drop`:
/// setters write `T`, removers replace with `()`, and the auto-derived
/// `Drop` handles both shapes.
///
/// [`finalize_or`](Self::finalize_or) is the one place that needs runtime
/// dispatch. For optional-with-default fields the trait method picks the
/// stored value or evaluates the default thunk; the dispatch is resolved
/// at monomorphization.
#[cfg_attr(docsrs, doc(notable_trait))]
pub trait Storage<T>: Satisfiable {
    /// The bag's actual field type for this flag — `T` for [`Yes`], `()` for
    /// [`No`].
    type Out;
    /// Yield the stored value when the flag is [`Yes`], evaluate the default
    /// thunk when it is [`No`].
    fn finalize_or<F: FnOnce() -> T>(stored: Self::Out, default: F) -> T;
}

/// Flag marker: the corresponding field has been set.
pub enum Yes {}
/// Flag marker: the corresponding field has not been set.
pub enum No {}

impl Satisfiable for Yes {
    const IS_SET: bool = true;
}
impl Satisfiable for No {
    const IS_SET: bool = false;
}
impl Satisfied for Yes {}

impl<T> Storage<T> for Yes {
    type Out = T;
    #[inline]
    fn finalize_or<F: FnOnce() -> T>(stored: T, _: F) -> T {
        stored
    }
}
impl<T> Storage<T> for No {
    type Out = ();
    #[inline]
    fn finalize_or<F: FnOnce() -> T>(_: (), default: F) -> T {
        default()
    }
}

mod sealed {
    pub trait Satisfiable {}
    impl Satisfiable for super::Yes {}
    impl Satisfiable for super::No {}

    pub trait Satisfied {}
    impl Satisfied for super::Yes {}
}
