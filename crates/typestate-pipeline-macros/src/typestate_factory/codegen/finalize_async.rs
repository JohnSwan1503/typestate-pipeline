use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, Visibility};

use crate::typestate_factory::{field::FieldInfo, spec::FinalizeAsyncSpec};

/// Emit `pub async fn finalize_async()` on the all-required-Yes bag.
///
/// Calls the inherent `finalize()` to assemble the raw struct, then awaits
/// the user-supplied `via` hook. Per-field bounds mirror `gen_finalize_sync`
/// so the impl is callable on the same set of bag shapes.
pub fn gen_finalize_async(
    bag: &Ident,
    vis: &Visibility,
    fields: &[FieldInfo],
    spec: &FinalizeAsyncSpec,
    no_unsafe: bool,
    prefix: &TokenStream2,
) -> TokenStream2 {
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

    let into = &spec.into;
    let via = &spec.via;
    let body = if let Some(err) = &spec.error {
        quote! {
            -> ::core::result::Result<#into, #err>
            {
                let raw = self.finalize();
                #via(raw).await
            }
        }
    } else {
        quote! {
            -> #into
            {
                let raw = self.finalize();
                #via(raw).await
            }
        }
    };

    quote! {
        impl< #( #impl_generics ),* > #bag< #( #flag_params ),* > {
            /// Async finalize hook. Calls `finalize()` to assemble the raw
            /// struct, then awaits the user-supplied hook.
            #vis async fn finalize_async(self) #body
        }
    }
}
