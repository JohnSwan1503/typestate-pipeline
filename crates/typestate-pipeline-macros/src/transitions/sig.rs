use proc_macro2::TokenStream as TokenStream2;
use proc_macro2_diagnostics::SpanDiagnosticExt;
use quote::{format_ident, quote};
use syn::{FnArg, Ident, Pat, PatIdent, Signature, spanned::Spanned};

use crate::diag::{MacroError, MacroResult};

pub(super) struct ParsedSig {
    pub has_ctx: bool,
    pub extras: Vec<TypedParam>,
}

#[derive(Clone)]
pub(super) struct TypedParam {
    pub name: Ident,
    pub original: FnArg,
}

impl ParsedSig {
    pub(super) fn parse(sig: &Signature, ctx_name: &Ident) -> MacroResult<Self> {
        let mut iter = sig.inputs.iter().peekable();
        let first = iter.next().ok_or_else(|| -> MacroError {
            sig.paren_token
                .span
                .join()
                .error("transition method must take `state: <Type>` as its first parameter")
                .help("transitions are written as if the body owned the state — add `state: <CurrentState>` as the first parameter")
                .into()
        })?;
        let state = parse_typed(first)?;
        if state.name != "state" {
            let actual = state.name.to_string();
            return Err(state
                .name
                .span()
                .error("transition method's first parameter must be named `state`")
                .help(format!("rename `{actual}` to `state` — the macro identifies the typestate-carrying parameter by name"))
                .into());
        }

        let mut has_ctx = false;
        if let Some(next) = iter.peek() {
            let candidate = parse_typed(next)?;
            if candidate.name == *ctx_name {
                has_ctx = true;
                let _ = iter.next();
            }
        }

        let mut extras = Vec::new();
        for arg in iter {
            extras.push(parse_typed(arg)?);
        }

        Ok(ParsedSig { has_ctx, extras })
    }

    pub(super) fn user_params(&self) -> Vec<FnArg> {
        self.extras.iter().map(|e| e.original.clone()).collect()
    }

    pub(super) fn body_call_args(&self) -> TokenStream2 {
        let mut parts: Vec<TokenStream2> = Vec::new();
        parts.push(quote! { state });
        if self.has_ctx {
            parts.push(quote! { ctx });
        }
        for e in &self.extras {
            let n = &e.name;
            parts.push(quote! { #n });
        }
        quote! { #( #parts ),* }
    }
}

pub(super) fn parse_typed(arg: &FnArg) -> syn::Result<TypedParam> {
    match arg {
        FnArg::Typed(pat_ty) => match &*pat_ty.pat {
            Pat::Ident(PatIdent {
                ident,
                by_ref: None,
                subpat: None,
                ..
            }) => Ok(TypedParam {
                name: ident.clone(),
                original: arg.clone(),
            }),
            other => Err(syn::Error::new(
                other.span(),
                "transition parameters must use simple `name: Type` patterns",
            )),
        },
        FnArg::Receiver(_) => Err(syn::Error::new(
            arg.span(),
            "transition methods take `state: <Type>` as their first parameter, not `self`",
        )),
    }
}

pub(super) fn body_fn_ident(method_name: &Ident) -> Ident {
    format_ident!("__{}_body", method_name)
}
