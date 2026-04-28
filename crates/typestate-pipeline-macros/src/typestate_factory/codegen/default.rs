use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

use crate::typestate_factory::{
    carrier::Carrier,
    field::FieldInfo,
    flags::{FlagPick, flag_args, other_field_bounds},
    pipeline_pair::{PipelinePairArgs, gen_pipeline_pair},
};

pub fn gen_default_helper(
    bag: &Ident,
    all: &[FieldInfo],
    this: &FieldInfo,
    carrier: Carrier<'_>,
    no_unsafe: bool,
    prefix: &TokenStream2,
) -> Option<TokenStream2> {
    let helper = this.default_helper_name.as_ref()?.clone();
    let default_expr = this.default_value.as_ref()?;
    let setter_name = &this.setter_name;
    let field_ty = &this.ty;
    let field_ident = &this.ident;
    let input_args = flag_args(all, &this.ident, FlagPick::No, prefix);
    let output_args = flag_args(all, &this.ident, FlagPick::Yes, prefix);
    let other_bounds = other_field_bounds(all, &this.ident, no_unsafe, prefix);

    // When the field has an `input = …` override the setter takes the
    // user's input type, not `field_ty`, so feeding the default through the
    // setter would either fail to type-check or run the transformer on the
    // default. Inline a direct field-write instead so the default is typed
    // as `field_ty` and bypasses the transformer entirely.
    let helper_body = if this.input_ty.is_some() {
        if no_unsafe {
            let field_moves = all.iter().map(|f| {
                let nm = &f.ident;
                if f.ident == *field_ident {
                    quote!( #nm: __tsh_field_value )
                } else {
                    quote!( #nm: __tsh_this.#nm )
                }
            });
            quote! {
                let __tsh_field_value: #field_ty = #default_expr;
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
                    if f.ident == *field_ident {
                        quote!( #nm: #prefix::__private::MaybeUninit::new(__tsh_field_value) )
                    } else {
                        quote!( #nm: unsafe { #prefix::__private::ptr::read(&__tsh_this.#nm) } )
                    }
                })
                .collect();
            quote! {
                let __tsh_field_value: #field_ty = #default_expr;
                let __tsh_this = #prefix::__private::ManuallyDrop::new(self);
                #bag {
                    #( #field_moves, )*
                    __tsh_markers: #prefix::__private::PhantomData,
                }
            }
        }
    } else {
        quote! {
            self.#setter_name(#default_expr)
        }
    };

    let standalone_arm = quote! {
        impl< #( #other_bounds ),* > #bag< #( #input_args ),* > {
            /// Apply the declared default and transition the flag to `Yes`.
            #[inline]
            pub fn #helper(self) -> #bag< #( #output_args ),* > {
                #helper_body
            }
        }
    };

    let result = match carrier {
        Carrier::Standalone => standalone_arm,
        Carrier::Pipeline(spec) => {
            let helper2 = helper.clone();
            gen_pipeline_pair(PipelinePairArgs {
                spec,
                method_name: &helper,
                params: &[],
                input_bag_ty: quote! { #bag< #( #input_args ),* > },
                output_bag_ty: quote! { #bag< #( #output_args ),* > },
                body_call: quote! { bag.#helper2() },
                fallible: false,
                is_async: false,
                error: None,
                extra_bounds: other_field_bounds(all, &this.ident, no_unsafe, prefix),
                prefix,
            })
        }
    };

    Some(result)
}
