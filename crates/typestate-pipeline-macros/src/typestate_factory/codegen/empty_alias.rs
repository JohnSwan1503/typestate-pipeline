use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Ident, Visibility};

use crate::typestate_factory::field::FieldInfo;

/// Emit a `<BagName>Empty` type alias for the all-`No` flag-tuple shape.
///
/// Mirrors the `<BagName>Ready` companion at the entry side: lets users
/// write
///
/// ```ignore
/// #[transition(into = SettingsEmpty)]
/// fn configure(state: Idle) -> SettingsEmpty {
///     Settings::new()
/// }
/// ```
///
/// instead of spelling out `Settings<No, No, No>` in two places. Internal
/// fields don't appear in the flag-generic list, so the alias's tuple
/// length equals the count of non-internal fields.
///
/// When the bag has zero non-internal fields the alias is just `pub type
/// <Bag>Empty = <Bag>;` (no angle brackets) — usable but generally
/// pointless, since there's no flag tuple to shorthand for.
pub fn gen_empty_alias(
    bag: &Ident,
    vis: &Visibility,
    fields: &[FieldInfo],
    prefix: &TokenStream2,
) -> TokenStream2 {
    let alias_name = format_ident!("{}Empty", bag);

    let no_count = fields.iter().filter(|f| !f.internal).count();
    if no_count == 0 {
        return quote! {
            /// Type alias for the bag's all-`No` flag-tuple shape.
            ///
            /// This bag has no flag-generic fields (every field is
            /// `#[field(internal)]`), so the alias is identical to the
            /// bag itself. Provided for API symmetry with bags that
            /// have flag generics.
            #vis type #alias_name = #bag;
        };
    }

    let no_params = (0..no_count).map(|_| quote!( #prefix::__private::No ));

    quote! {
        /// Type alias for the bag's all-`No` flag-tuple shape — the
        /// fresh, no-fields-set form returned by `<Bag>::new(…)`.
        ///
        /// Use as the entry-side counterpart to [`<Bag>Ready`] at the
        /// exit: `#[transition(into = <Bag>Empty)]` is shorter and more
        /// readable than spelling out the full `<No, No, …>` tuple.
        #vis type #alias_name = #bag< #( #no_params ),* >;
    }
}
