use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Ident;

use crate::typestate_factory::{
    carrier::Carrier,
    field::FieldInfo,
    flags::{FlagPick, flag_args, other_field_bounds},
    pipeline_pair::{PipelinePairArgs, gen_pipeline_pair},
};

pub fn gen_remover(
    bag: &Ident,
    all: &[FieldInfo],
    this: &FieldInfo,
    carrier: Carrier<'_>,
    no_unsafe: bool,
    prefix: &TokenStream2,
) -> TokenStream2 {
    let remover_name = format_ident!("drop_{}", this.ident);
    let input_args = flag_args(all, &this.ident, FlagPick::Yes, prefix);
    let output_args = flag_args(all, &this.ident, FlagPick::No, prefix);

    let n = &this.ident;

    let build_bag = if no_unsafe {
        // The OLD value of the target slot drops as a leftover un-moved
        // field at end-of-scope — no manual drop needed.
        let field_moves = all.iter().map(|f| {
            let nm = &f.ident;
            if f.ident == this.ident {
                quote!( #nm: () )
            } else {
                quote!( #nm: __tsh_this.#nm )
            }
        });
        quote! {
            let __tsh_this = self;
            #bag {
                #( #field_moves, )*
                __tsh_markers: #prefix::__private::PhantomData,
            }
        }
    } else {
        let field_moves = all.iter().map(|f| {
            let nm = &f.ident;
            if f.ident == this.ident {
                quote!( #nm: #prefix::__private::MaybeUninit::uninit() )
            } else {
                quote!( #nm: unsafe { #prefix::__private::ptr::read(&__tsh_this.#nm) } )
            }
        });
        quote! {
            let __tsh_this = #prefix::__private::ManuallyDrop::new(self);
            // SAFETY: input bag has flag = Yes, so this field is initialized.
            // Move it to a stack temp so it auto-drops at end-of-scope rather
            // than mid-body — a panicking `T::drop` then unwinds with the
            // new bag already constructed, and that bag's panic-safe `Drop`
            // reclaims the other set fields. (See overrider.rs for the
            // mirror invariant.)
            let __tsh_old_field = unsafe { __tsh_this.#n.assume_init_read() };
            let __tsh_new_bag = #bag {
                #( #field_moves, )*
                __tsh_markers: #prefix::__private::PhantomData,
            };
            ::core::mem::drop(__tsh_old_field);
            __tsh_new_bag
        }
    };

    let standalone_arm = {
        let other_bounds = other_field_bounds(all, &this.ident, no_unsafe, prefix);
        quote! {
            impl< #( #other_bounds ),* > #bag< #( #input_args ),* > {
                /// Drop the field's value, transitioning the flag back to `No`.
                #[inline]
                #[allow(non_snake_case)]
                pub fn #remover_name(self) -> #bag< #( #output_args ),* > {
                    #build_bag
                }
            }
        }
    };

    match carrier {
        Carrier::Standalone => standalone_arm,
        Carrier::Pipeline(spec) => {
            let body_call = {
                let r = remover_name.clone();
                quote! { bag.#r() }
            };
            gen_pipeline_pair(PipelinePairArgs {
                spec,
                method_name: &remover_name,
                params: &[],
                input_bag_ty: quote! { #bag< #( #input_args ),* > },
                output_bag_ty: quote! { #bag< #( #output_args ),* > },
                body_call,
                fallible: false,
                is_async: false,
                error: None,
                extra_bounds: other_field_bounds(all, &this.ident, no_unsafe, prefix),
                prefix,
            })
        }
    }
}
