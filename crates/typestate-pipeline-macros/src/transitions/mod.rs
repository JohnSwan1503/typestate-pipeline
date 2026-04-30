//! `#[transitions]` attribute macro.
//!
//! Decorates an `impl` block whose methods are typestate transitions. Each
//! method marked `#[transition(into = NextState)]` has its body expanded
//! into a Resolved + InFlight method pair on the underlying `Pipeline`
//! carrier.

use proc_macro2::TokenStream as TokenStream2;
use proc_macro2_diagnostics::SpanDiagnosticExt;
use quote::ToTokens;
use syn::{Attribute, Ident, ImplItem, ItemImpl, Type, spanned::Spanned};

use crate::{
    diag::MacroResult,
    prefix::facade_path,
};

mod args;
mod codegen;
mod self_ty;
mod sig;
mod spec;

pub(crate) use args::TransitionsArgs;

use args::TransitionArgs;
use spec::TransitionSpec;

pub(crate) fn expand(args: TransitionsArgs, mut input: ItemImpl) -> MacroResult<TokenStream2> {
    if let Some((_, trait_path, _)) = input.trait_.as_ref() {
        return Err(trait_path
            .span()
            .error("#[transitions] can only be applied to inherent impl blocks")
            .help("move these methods to an inherent `impl <Carrier>` block; the macro expands every `#[transition]` into a Resolved + InFlight method pair on the carrier and that requires inherent self")
            .into());
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
                    let attr_span = attr.span();
                    let targs: TransitionArgs = attr.parse_args()?;
                    transition_specs.push(TransitionSpec::from_fn(f, targs, attr_span)?);
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

fn extract_carrier_ident(self_ty: &Type) -> MacroResult<Ident> {
    let Type::Path(tp) = self_ty else {
        return Err(syn::Error::new(
            self_ty.span(),
            "#[transitions] expects the impl's self type to be a path like `MyCarrier<...>`",
        )
        .into());
    };
    let Some(last) = tp.path.segments.last() else {
        return Err(syn::Error::new(
            self_ty.span(),
            "#[transitions] expects the impl's self type to be a non-empty path",
        )
        .into());
    };
    Ok(last.ident.clone())
}

fn take_transition_attr(attrs: &mut Vec<Attribute>) -> Option<Attribute> {
    let idx = attrs.iter().position(|a| a.path().is_ident("transition"))?;
    Some(attrs.remove(idx))
}
