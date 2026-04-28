use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, Visibility};

use crate::typestate_factory::field::FieldInfo;

/// Emit `fn new(...)` and a `Default` impl when applicable.
///
/// `new` takes one positional parameter per *internal* field in declaration
/// order. Non-internal fields start with their flag generic = `No`.
///
/// `Default` is only emitted when there are no internal fields. With
/// internal fields the constructor takes positional args, so there's no
/// canonical "empty" form for `Default::default()` to materialize.
pub fn gen_new_impl(
    bag: &Ident,
    vis: &Visibility,
    fields: &[FieldInfo],
    no_unsafe: bool,
    prefix: &TokenStream2,
) -> TokenStream2 {
    let init_flag_args: Vec<TokenStream2> = fields
        .iter()
        .filter(|f| !f.internal)
        .map(|_| quote!(#prefix::__private::No))
        .collect();

    let new_params: Vec<TokenStream2> = fields
        .iter()
        .filter(|f| f.internal)
        .map(|f| {
            let n = &f.ident;
            let t = &f.ty;
            quote! { #n: #t }
        })
        .collect();

    let field_inits = fields.iter().map(|f| {
        let n = &f.ident;
        if f.internal {
            quote! { #n }
        } else if no_unsafe {
            quote! { #n: () }
        } else {
            quote! { #n: #prefix::__private::MaybeUninit::uninit() }
        }
    });

    let new_impl = quote! {
        impl #bag< #( #init_flag_args ),* > {
            /// Construct a fresh bag.
            ///
            /// Internal fields (declared with `#[field(internal)]`) are
            /// supplied positionally. Non-internal fields start unset.
            #vis fn new( #( #new_params ),* ) -> Self {
                Self {
                    #( #field_inits, )*
                    __tsh_markers: #prefix::__private::PhantomData,
                }
            }
        }
    };

    let default_impl = if fields.iter().any(|f| f.internal) {
        TokenStream2::new()
    } else {
        let default_flag_args = init_flag_args.clone();
        quote! {
            impl ::core::default::Default for #bag< #( #default_flag_args ),* > {
                fn default() -> Self {
                    Self::new()
                }
            }
        }
    };

    quote! {
        #new_impl
        #default_impl
    }
}
