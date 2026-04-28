use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{Attribute, Block, FnArg, Ident, ItemImpl};

use super::{
    args::TransitionsArgs,
    self_ty::{first_lifetime, merge_generics, pipeline_self_ty, Mode},
    sig::{body_fn_ident, ParsedSig},
    spec::{BodyShape, TransitionSpec},
};

impl TransitionSpec {
    pub(super) fn expand(
        &self,
        template: &ItemImpl,
        top: &TransitionsArgs,
        carrier: &Ident,
        prefix: &TokenStream2,
    ) -> syn::Result<TokenStream2> {
        let parsed = ParsedSig::parse(&self.method.sig, &top.ctx)?;
        let body_fn = self.build_body_fn(&parsed, template);
        let resolved_arm =
            self.build_arm(Mode::Resolved, &parsed, template, top, carrier, prefix)?;
        let inflight_arm =
            self.build_arm(Mode::InFlight, &parsed, template, top, carrier, prefix)?;
        Ok(quote! {
            #body_fn
            #resolved_arm
            #inflight_arm
        })
    }

    fn build_body_fn(&self, parsed: &ParsedSig, template: &ItemImpl) -> TokenStream2 {
        let body_name = body_fn_ident(&self.method.sig.ident);
        let asyncness = &self.method.sig.asyncness;
        let body: &Block = &self.method.block;
        let return_type = &self.method.sig.output;

        let merged = merge_generics(&template.generics, &self.method.sig.generics);
        let (impl_generics, _, where_clause) = merged.split_for_impl();
        let params = self.body_params_for(parsed);

        // `unused_variables` covers the common case where the user's body
        // doesn't reference its `state: SomeMarkerState` first parameter —
        // saves the user a `let _ = state;` ritual. `extra_unused_lifetimes`
        // fires when the impl carries `'a` but the body fn signature does
        // not reference it (carrier-less bodies).
        quote! {
            #[doc(hidden)]
            #[allow(
                non_snake_case,
                unused_variables,
                clippy::too_many_arguments,
                clippy::extra_unused_lifetimes,
            )]
            #asyncness fn #body_name #impl_generics ( #( #params ),* ) #return_type
            #where_clause
            #body
        }
    }

    fn body_params_for(&self, parsed: &ParsedSig) -> Vec<FnArg> {
        let mut v: Vec<FnArg> = Vec::new();
        let mut iter = self.method.sig.inputs.iter();
        if let Some(arg) = iter.next() {
            v.push(arg.clone());
        }
        if parsed.has_ctx && let Some(arg) = iter.next() {
            v.push(arg.clone());
        }
        for arg in iter {
            v.push(arg.clone());
        }
        v
    }

    fn build_arm(
        &self,
        input_mode: Mode,
        parsed: &ParsedSig,
        template: &ItemImpl,
        top: &TransitionsArgs,
        carrier: &Ident,
        prefix: &TokenStream2,
    ) -> syn::Result<TokenStream2> {
        let method_name = &self.method.sig.ident;
        let body_name = body_fn_ident(method_name);
        let vis = &self.method.vis;
        let outer_attrs: Vec<&Attribute> = self.method.attrs.iter().collect();
        // Forward impl-block-level attrs to each generated arm so things like
        // `#[allow(clippy::needless_lifetimes)]` placed on the user's impl
        // reach the macro-emitted impls.
        let impl_attrs: Vec<&Attribute> = template.attrs.iter().collect();
        let user_params = parsed.user_params();
        let body_call_args = parsed.body_call_args();

        let (impl_generics, _, where_clause) = template.generics.split_for_impl();
        let self_ty = pipeline_self_ty(&template.self_ty, input_mode, prefix);

        let lifetime = first_lifetime(&template.generics)
            .map(|lt| quote!(#lt))
            .unwrap_or_else(|| quote!('_));
        let pipelined = quote! { #prefix::__private::Pipelined<#lifetime> };
        let next_state = &self.args.into;
        let dest_ty = |out_mode: Mode| -> TokenStream2 {
            match out_mode {
                Mode::Resolved => quote! { <Self as #pipelined>::Resolved<#next_state> },
                Mode::InFlight => quote! { <Self as #pipelined>::InFlight<#next_state> },
            }
        };
        let err: TokenStream2 = match &top.error {
            Some(e) => quote!(#e),
            None => quote! { <Self as #pipelined>::Error },
        };

        let unwrap = match input_mode {
            Mode::Resolved => quote! {
                let (ctx, state) = self.0.into_parts();
            },
            Mode::InFlight => quote! {
                let (ctx, pending) = self.0.into_parts();
            },
        };

        let body_call = quote! { #body_name(#body_call_args) };

        let wrap_resolved = |value: TokenStream2| {
            quote! { #carrier(#prefix::__private::Pipeline::resolved(ctx, #value)) }
        };
        let wrap_inflight = |future_body: TokenStream2| {
            quote! {
                #carrier(#prefix::__private::Pipeline::in_flight(
                    ctx,
                    #prefix::__private::Box::pin(async move { #future_body }),
                ))
            }
        };

        let (asyncness, return_type, body): (TokenStream2, TokenStream2, TokenStream2) =
            match (self.shape, input_mode) {
                (BodyShape::SyncInfallible, Mode::Resolved) => {
                    let dest = dest_ty(Mode::Resolved);
                    let out = wrap_resolved(body_call.clone());
                    (quote!(), quote!(#dest), quote! { #unwrap #out })
                }
                (BodyShape::SyncInfallible, Mode::InFlight) => {
                    let dest = dest_ty(Mode::InFlight);
                    let out = wrap_inflight(quote! {
                        let state = pending.await?;
                        ::core::result::Result::Ok(#body_call)
                    });
                    (quote!(), quote!(#dest), quote! { #unwrap #out })
                }
                (BodyShape::SyncFallible, Mode::Resolved) => {
                    let dest = dest_ty(Mode::Resolved);
                    let out = wrap_resolved(quote!(next));
                    let body = quote! {
                        #unwrap
                        let next = #body_call?;
                        ::core::result::Result::Ok(#out)
                    };
                    (quote!(), quote!(::core::result::Result<#dest, #err>), body)
                }
                (BodyShape::SyncFallible, Mode::InFlight) => {
                    let dest = dest_ty(Mode::InFlight);
                    let out = wrap_inflight(quote! {
                        let state = pending.await?;
                        #body_call
                    });
                    (quote!(), quote!(#dest), quote! { #unwrap #out })
                }
                (BodyShape::AsyncDeferred, Mode::Resolved) => {
                    let dest = dest_ty(Mode::InFlight);
                    let out = wrap_inflight(quote! { #body_call.await });
                    (quote!(), quote!(#dest), quote! { #unwrap #out })
                }
                (BodyShape::AsyncDeferred, Mode::InFlight) => {
                    let dest = dest_ty(Mode::InFlight);
                    let out = wrap_inflight(quote! {
                        let state = pending.await?;
                        #body_call.await
                    });
                    (quote!(), quote!(#dest), quote! { #unwrap #out })
                }
                (BodyShape::AsyncBreakpoint, Mode::Resolved) => {
                    let dest = dest_ty(Mode::Resolved);
                    let out = wrap_resolved(quote!(next));
                    let body = quote! {
                        #unwrap
                        let next = #body_call.await?;
                        ::core::result::Result::Ok(#out)
                    };
                    (quote!(async), quote!(::core::result::Result<#dest, #err>), body)
                }
                (BodyShape::AsyncBreakpoint, Mode::InFlight) => {
                    let dest = dest_ty(Mode::Resolved);
                    let out = wrap_resolved(quote!(next));
                    let body = quote! {
                        #unwrap
                        let state = pending.await?;
                        let next = #body_call.await?;
                        ::core::result::Result::Ok(#out)
                    };
                    (quote!(async), quote!(::core::result::Result<#dest, #err>), body)
                }
            };

        let mut output = TokenStream2::new();
        for attr in &impl_attrs {
            attr.to_tokens(&mut output);
        }
        let impl_block = quote! {
            impl #impl_generics #self_ty #where_clause {
                #( #outer_attrs )*
                #vis #asyncness fn #method_name(self, #( #user_params ),*) -> #return_type {
                    #body
                }
            }
        };
        impl_block.to_tokens(&mut output);
        Ok(output)
    }
}
