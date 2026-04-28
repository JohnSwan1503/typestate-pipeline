use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, Type};

use crate::typestate_factory::spec::PipelineSpec;

/// Inputs for [`gen_pipeline_pair`]. Bundling them into a struct keeps call
/// sites readable (named fields) and avoids the "swap the wrong literal"
/// hazard of a long positional-arg list.
pub struct PipelinePairArgs<'a> {
    pub spec: &'a PipelineSpec,
    pub method_name: &'a Ident,
    pub params: &'a [(TokenStream2, Type)],
    pub input_bag_ty: TokenStream2,
    pub output_bag_ty: TokenStream2,
    pub body_call: TokenStream2,
    pub fallible: bool,
    pub is_async: bool,
    pub error: Option<&'a Type>,
    /// Bounds to add to the impl-block's generics. Caller collects from
    /// `other_field_bounds(...)`.
    pub extra_bounds: Vec<TokenStream2>,
    pub prefix: &'a TokenStream2,
}

/// Emit a Resolved + InFlight pair on the user's carrier that mirrors a
/// standalone bag transition.
///
/// - Resolved arm: extract bag from `self.0`, call the standalone method,
///   re-wrap.
/// - InFlight arm: chain through the pending future.
pub fn gen_pipeline_pair(args: PipelinePairArgs<'_>) -> TokenStream2 {
    let PipelinePairArgs {
        spec,
        method_name,
        params,
        input_bag_ty,
        output_bag_ty,
        body_call,
        fallible,
        is_async,
        error,
        extra_bounds,
        prefix,
    } = args;

    let carrier = &spec.carrier;
    // The InFlight `Mode` impl requires `S: Send + 'a`. `Yes`/`No` are
    // zero-sized and `Send + 'static` so these bounds always trivially hold;
    // we add them so the InFlight impl block compiles.
    let extra_bounds: Vec<TokenStream2> = extra_bounds
        .into_iter()
        .map(|b| quote!( #b + ::core::marker::Send + 'a ))
        .collect();
    let extra_bounds2 = extra_bounds.clone();

    let param_decls: Vec<TokenStream2> = params
        .iter()
        .map(|(name, ty)| quote! { #name: #ty })
        .collect();
    let param_decls2 = param_decls.clone();

    let pending_body_inner = if fallible {
        quote! { #body_call }
    } else {
        quote! { ::core::result::Result::Ok(#body_call) }
    };

    // The async / InFlight arms box the body into a `BoxFuture`, which
    // requires the captured bag (and the bag the body produces) to be
    // `Send`. Surfacing those obligations in an explicit `where` clause
    // points the diagnostic at the impl block when a user's field type
    // is non-`Send`, instead of leaving rustc to discover it deep inside
    // `Box::pin`'s instantiation.
    let send_where = quote! {
        where
            #input_bag_ty: ::core::marker::Send + 'a,
            #output_bag_ty: ::core::marker::Send + 'a,
    };

    let resolved = if is_async {
        quote! {
            impl<'a, #( #extra_bounds ),* >
                #carrier<'a, #input_bag_ty, #prefix::__private::Resolved>
            #send_where
            {
                #[inline]
                pub fn #method_name(self, #( #param_decls ),*)
                    -> #carrier<'a, #output_bag_ty, #prefix::__private::InFlight>
                {
                    let (ctx, bag) = self.0.into_parts();
                    #carrier(#prefix::__private::Pipeline::in_flight(
                        ctx,
                        #prefix::__private::Box::pin(async move {
                            #pending_body_inner
                        }),
                    ))
                }
            }
        }
    } else if fallible {
        let err = error.expect("validated upstream");
        quote! {
            impl<'a, #( #extra_bounds ),* >
                #carrier<'a, #input_bag_ty, #prefix::__private::Resolved>
            {
                #[inline]
                pub fn #method_name(self, #( #param_decls ),*)
                    -> ::core::result::Result<
                        #carrier<'a, #output_bag_ty, #prefix::__private::Resolved>,
                        #err,
                    >
                {
                    let (ctx, bag) = self.0.into_parts();
                    let next = #body_call?;
                    ::core::result::Result::Ok(#carrier(
                        #prefix::__private::Pipeline::resolved(ctx, next),
                    ))
                }
            }
        }
    } else {
        quote! {
            impl<'a, #( #extra_bounds ),* >
                #carrier<'a, #input_bag_ty, #prefix::__private::Resolved>
            {
                #[inline]
                pub fn #method_name(self, #( #param_decls ),*)
                    -> #carrier<'a, #output_bag_ty, #prefix::__private::Resolved>
                {
                    let (ctx, bag) = self.0.into_parts();
                    let next = #body_call;
                    #carrier(#prefix::__private::Pipeline::resolved(ctx, next))
                }
            }
        }
    };

    let inflight = quote! {
        impl<'a, #( #extra_bounds2 ),* >
            #carrier<'a, #input_bag_ty, #prefix::__private::InFlight>
        #send_where
        {
            #[inline]
            pub fn #method_name(self, #( #param_decls2 ),*)
                -> #carrier<'a, #output_bag_ty, #prefix::__private::InFlight>
            {
                let (ctx, pending) = self.0.into_parts();
                #carrier(#prefix::__private::Pipeline::in_flight(
                    ctx,
                    #prefix::__private::Box::pin(async move {
                        let bag = pending.await?;
                        #pending_body_inner
                    }),
                ))
            }
        }
    };

    quote! {
        #resolved
        #inflight
    }
}
