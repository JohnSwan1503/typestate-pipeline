use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, Type};

use crate::typestate_factory::{
    carrier::Carrier,
    field::FieldInfo,
    flags::{FlagPick, flag_args, other_field_bounds},
    pipeline_pair::{PipelinePairArgs, gen_pipeline_pair},
};

pub fn gen_setter(
    bag: &Ident,
    all: &[FieldInfo],
    this: &FieldInfo,
    error: Option<&Type>,
    carrier: Carrier<'_>,
    no_unsafe: bool,
    prefix: &TokenStream2,
) -> TokenStream2 {
    let input_args = flag_args(all, &this.ident, FlagPick::No, prefix);
    let output_args = flag_args(all, &this.ident, FlagPick::Yes, prefix);

    let setter_name = &this.setter_name;
    let field_ty = &this.ty;
    let setter_input_ty: &Type = this.input_ty.as_ref().unwrap_or(field_ty);

    let make_new_value = match (this.setter_fn.as_ref(), this.async_fn, this.fallible) {
        (None, _, _) => quote! { val },
        (Some(f), false, false) => quote! { #f(val) },
        (Some(f), false, true) => quote! { #f(val)? },
        (Some(f), true, false) => quote! { #f(val).await },
        (Some(f), true, true) => quote! { #f(val).await? },
    };

    let field_moves: Vec<TokenStream2> = all
        .iter()
        .map(|f| {
            let nm = &f.ident;
            if f.ident == this.ident {
                if no_unsafe {
                    quote!( #nm: __tsh_field_value )
                } else {
                    quote!( #nm: #prefix::__private::MaybeUninit::new(__tsh_field_value) )
                }
            } else if no_unsafe {
                quote!( #nm: __tsh_this.#nm )
            } else {
                quote!( #nm: unsafe { #prefix::__private::ptr::read(&__tsh_this.#nm) } )
            }
        })
        .collect();

    // Compute the (possibly fallible / async) value FIRST, before binding
    // `self`. A failing `?` or dropped future then leaves `self` whole and
    // its `Drop` runs normally. Reordering would suppress that drop (unsafe
    // mode: ManuallyDrop already wraps; safe mode: partial moves already
    // started) and leak the other set fields. The `factory_no_leak` tests
    // are the regression guard.
    let bind_self = if no_unsafe {
        quote!( let __tsh_this = self; )
    } else {
        quote!( let __tsh_this = #prefix::__private::ManuallyDrop::new(self); )
    };
    let build_bag = quote! {
        let __tsh_field_value = #make_new_value;
        #bind_self
        #bag {
            #( #field_moves, )*
            __tsh_markers: #prefix::__private::PhantomData,
        }
    };

    let asyncness = if this.async_fn {
        quote!(async)
    } else {
        quote!()
    };
    let standalone_arm = {
        let other_bounds = other_field_bounds(all, &this.ident, no_unsafe, prefix);
        if this.fallible {
            let err = error.expect("validated upstream");
            quote! {
                impl< #( #other_bounds ),* > #bag< #( #input_args ),* > {
                    #[inline]
                    pub #asyncness fn #setter_name(self, val: #setter_input_ty)
                        -> ::core::result::Result<#bag< #( #output_args ),* >, #err>
                    {
                        ::core::result::Result::Ok({ #build_bag })
                    }
                }
            }
        } else {
            quote! {
                impl< #( #other_bounds ),* > #bag< #( #input_args ),* > {
                    #[inline]
                    pub #asyncness fn #setter_name(self, val: #setter_input_ty) -> #bag< #( #output_args ),* > {
                        #build_bag
                    }
                }
            }
        }
    };

    match carrier {
        Carrier::Standalone => standalone_arm,
        Carrier::Pipeline(spec) => {
            let body_call = {
                let s = &this.setter_name;
                if this.async_fn {
                    quote! { bag.#s(val).await }
                } else {
                    quote! { bag.#s(val) }
                }
            };
            gen_pipeline_pair(PipelinePairArgs {
                spec,
                method_name: &this.setter_name,
                params: &[(quote!(val), setter_input_ty.clone())],
                input_bag_ty: quote! { #bag< #( #input_args ),* > },
                output_bag_ty: quote! { #bag< #( #output_args ),* > },
                body_call,
                fallible: this.fallible,
                is_async: this.async_fn,
                error,
                extra_bounds: other_field_bounds(all, &this.ident, no_unsafe, prefix),
                prefix,
            })
        }
    }
}
