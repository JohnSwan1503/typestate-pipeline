//! `#[transitions]` attribute macro.
//!
//! Decorates an `impl` block whose methods are typestate transitions. Each
//! method marked `#[transition(into = NextState)]` has its body expanded
//! into a Resolved + InFlight method pair on the underlying `Pipeline`
//! carrier.

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{Attribute, Ident, ImplItem, ItemImpl, Type, spanned::Spanned};

use crate::prefix::facade_path;

mod args;
mod codegen;
mod self_ty;
mod sig;
mod spec;

pub(crate) use args::TransitionsArgs;

use args::TransitionArgs;
use spec::TransitionSpec;

pub(crate) fn expand(args: TransitionsArgs, mut input: ItemImpl) -> syn::Result<TokenStream2> {
    if input.trait_.is_some() {
        return Err(syn::Error::new(
            input.span(),
            "#[transitions] can only be applied to inherent impl blocks",
        ));
    }

    // The user's carrier must be a tuple-struct newtype around `Pipeline`;
    // orphan rules forbid inherent impls on type aliases of `Pipeline` itself.
    let carrier = extract_carrier_ident(&input.self_ty)?;
    let prefix = facade_path();

    let mut transition_specs: Vec<TransitionSpec> = Vec::new();
    let mut passthrough: Vec<ImplItem> = Vec::new();

    let items = std::mem::take(&mut input.items);
    for item in items {
        match item {
            ImplItem::Fn(mut f) => {
                if let Some(attr) = take_transition_attr(&mut f.attrs) {
                    let targs: TransitionArgs = attr.parse_args()?;
                    transition_specs.push(TransitionSpec::from_fn(f, targs)?);
                } else {
                    passthrough.push(ImplItem::Fn(f));
                }
            }
            other => passthrough.push(other),
        }
    }

    let mut output = TokenStream2::new();

    if !passthrough.is_empty() {
        let mut pass = input.clone();
        pass.items = passthrough;
        pass.to_tokens(&mut output);
    }

    for spec in transition_specs {
        let impls = spec.expand(&input, &args, &carrier, &prefix)?;
        impls.to_tokens(&mut output);
    }

    Ok(output)
}

fn extract_carrier_ident(self_ty: &Type) -> syn::Result<Ident> {
    let Type::Path(tp) = self_ty else {
        return Err(syn::Error::new(
            self_ty.span(),
            "#[transitions] expects the impl's self type to be a path like `MyCarrier<...>`",
        ));
    };
    let Some(last) = tp.path.segments.last() else {
        return Err(syn::Error::new(
            self_ty.span(),
            "#[transitions] expects the impl's self type to be a non-empty path",
        ));
    };
    Ok(last.ident.clone())
}

fn take_transition_attr(attrs: &mut Vec<Attribute>) -> Option<Attribute> {
    let idx = attrs.iter().position(|a| a.path().is_ident("transition"))?;
    Some(attrs.remove(idx))
}
