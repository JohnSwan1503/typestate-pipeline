use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Ident, Type};

use crate::typestate_factory::{
    carrier::Carrier,
    field::FieldInfo,
    flags::{FlagPick, flag_args, other_field_bounds},
    pipeline_pair::{PipelinePairArgs, gen_pipeline_pair},
};

pub fn gen_overrider(
    bag: &Ident,
    all: &[FieldInfo],
    this: &FieldInfo,
    error: Option<&Type>,
    carrier: Carrier<'_>,
    no_unsafe: bool,
    prefix: &TokenStream2,
) -> TokenStream2 {
    let override_name = format_ident!("override_{}", this.ident);
    let yes_args = flag_args(all, &this.ident, FlagPick::Yes, prefix);
    let yes_args_out = yes_args.clone();
    let other_bounds = other_field_bounds(all, &this.ident, no_unsafe, prefix);

    let n_field = &this.ident;
    let field_ty = &this.ty;
    let setter_input_ty: &Type = this.input_ty.as_ref().unwrap_or(field_ty);

    let make_new_value = match (this.setter_fn.as_ref(), this.async_fn, this.fallible) {
        (None, _, _) => quote! { val },
        (Some(f), false, false) => quote! { #f(val) },
        (Some(f), false, true) => quote! { #f(val)? },
        (Some(f), true, false) => quote! { #f(val).await },
        (Some(f), true, true) => quote! { #f(val).await? },
    };

    // Compute the new value FIRST so a failing `?` or dropped future
    // leaves `self` live and its drop releases the old field plus every
    // other set field. See setter.rs for the same invariant.
    //
    // Unsafe mode also reads the OLD value into a stack temp instead of
    // dropping it in-place. End-of-scope auto-drop handles the temp,
    // which means a panicking `T::drop` on the old value unwinds with
    // the new bag already constructed — and the new bag's panic-safe
    // `Drop` impl reclaims the other set fields. (Calling
    // `assume_init_drop` mid-body would panic *before* the new bag
    // existed, leaving the other `MaybeUninit` slots inside `this`
    // (which is `ManuallyDrop`) to leak.)
    let build_bag = if no_unsafe {
        // The OLD value drops as the leftover un-moved slot at end-of-scope.
        let field_moves = all.iter().map(|f| {
            let nm = &f.ident;
            if f.ident == this.ident {
                quote!( #nm: __tsh_field_value )
            } else {
                quote!( #nm: __tsh_this.#nm )
            }
        });
        quote! {
            let __tsh_field_value = #make_new_value;
            let __tsh_this = self;
            #bag {
                #( #field_moves, )*
                __tsh_markers: #prefix::__private::PhantomData,
            }
        }
    } else {
        let field_moves: Vec<TokenStream2> = all
            .iter()
            .map(|f| {
                let nm = &f.ident;
                if f.ident == this.ident {
                    quote!( #nm: #prefix::__private::MaybeUninit::new(__tsh_field_value) )
                } else {
                    quote!( #nm: unsafe { #prefix::__private::ptr::read(&__tsh_this.#nm) } )
                }
            })
            .collect();
        quote! {
            let __tsh_field_value = #make_new_value;
            let __tsh_this = #prefix::__private::ManuallyDrop::new(self);
            // SAFETY: input bag has flag = Yes for this field, so it's
            // initialized. Move it to a stack temp so it auto-drops at
            // end-of-scope rather than mid-body.
            let __tsh_old_field = unsafe { __tsh_this.#n_field.assume_init_read() };
            let __tsh_new_bag = #bag {
                #( #field_moves, )*
                __tsh_markers: #prefix::__private::PhantomData,
            };
            // `__tsh_old_field` drops here on normal return. On unwind,
            // the already-built `__tsh_new_bag` drops via its panic-safe
            // `Drop`.
            ::core::mem::drop(__tsh_old_field);
            __tsh_new_bag
        }
    };

    let asyncness = if this.async_fn {
        quote!(async)
    } else {
        quote!()
    };
    let standalone_arm = if this.fallible {
        let err = error.expect("validated upstream");
        quote! {
            impl< #( #other_bounds ),* > #bag< #( #yes_args ),* > {
                /// Replace the existing value. Stays in the `Yes` state.
                #[inline]
                #[allow(non_snake_case)]
                pub #asyncness fn #override_name(self, val: #setter_input_ty)
                    -> ::core::result::Result<#bag< #( #yes_args_out ),* >, #err>
                {
                    ::core::result::Result::Ok({ #build_bag })
                }
            }
        }
    } else {
        quote! {
            impl< #( #other_bounds ),* > #bag< #( #yes_args ),* > {
                /// Replace the existing value. Stays in the `Yes` state.
                #[inline]
                #[allow(non_snake_case)]
                pub #asyncness fn #override_name(self, val: #setter_input_ty) -> #bag< #( #yes_args_out ),* > {
                    #build_bag
                }
            }
        }
    };

    match carrier {
        Carrier::Standalone => standalone_arm,
        Carrier::Pipeline(spec) => {
            let body_call = {
                let nm = override_name.clone();
                if this.async_fn {
                    quote! { bag.#nm(val).await }
                } else {
                    quote! { bag.#nm(val) }
                }
            };
            gen_pipeline_pair(PipelinePairArgs {
                spec,
                method_name: &override_name,
                params: &[(quote!(val), setter_input_ty.clone())],
                input_bag_ty: quote! { #bag< #( #yes_args ),* > },
                output_bag_ty: quote! { #bag< #( #yes_args_out ),* > },
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
