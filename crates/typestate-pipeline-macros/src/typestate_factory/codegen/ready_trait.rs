use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Ident, Visibility};

use crate::typestate_factory::field::FieldInfo;

/// Emit a `<BagName>Ready` trait + auto-impl that abstracts over the bag's
/// finalize-callable shape. Lets users write `where B: <BagName>Ready` as a
/// shorthand for the explicit flag tuple plus per-flag bounds.
///
/// The trait method is `into_finalized` (not `finalize`) to avoid
/// method-resolution ambiguity in the auto-impl body — calling
/// `self.finalize()` there would otherwise recurse into the trait method
/// instead of dispatching to the inherent.
///
/// Per-field bounds mirror `gen_finalize_sync` exactly.
pub fn gen_ready_trait(
    bag: &Ident,
    original: &Ident,
    vis: &Visibility,
    fields: &[FieldInfo],
    no_unsafe: bool,
    prefix: &TokenStream2,
) -> TokenStream2 {
    let trait_name = format_ident!("{}Ready", bag);

    let mut flag_params: Vec<TokenStream2> = Vec::new();
    let mut impl_generics: Vec<TokenStream2> = Vec::new();
    for f in fields {
        if f.internal {
            continue;
        }
        let p = &f.flag_param;
        let t = &f.ty;
        let optional_with_default = !f.required && f.default_value.is_some();
        if optional_with_default {
            if no_unsafe {
                impl_generics.push(quote!( #p: #prefix::__private::Storage<#t> ));
            } else {
                impl_generics.push(quote!( #p: #prefix::__private::Satisfiable ));
            }
            flag_params.push(quote!( #p ));
        } else if no_unsafe {
            flag_params.push(quote!( #prefix::__private::Yes ));
        } else {
            impl_generics.push(quote!( #p: #prefix::__private::Satisfied ));
            flag_params.push(quote!( #p ));
        }
    }

    quote! {
        /// Trait shorthand for "this bag is in a finalize-callable shape".
        ///
        /// Auto-implemented for any flag tuple where required fields are
        /// `Yes` and optional-with-default fields can be either. Use as a
        /// where-clause bound to abstract over the concrete flag tuple at
        /// finalize-bound impl sites.
        #vis trait #trait_name: ::core::marker::Sized {
            /// Consume the bag and assemble the finalized struct.
            ///
            /// Mirrors the inherent `finalize()` exactly.
            fn into_finalized(self) -> #original;
        }

        impl< #( #impl_generics ),* > #trait_name for #bag< #( #flag_params ),* > {
            #[inline]
            fn into_finalized(self) -> #original {
                self.finalize()
            }
        }
    }
}
