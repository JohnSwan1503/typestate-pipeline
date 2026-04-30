use proc_macro2::TokenStream as TokenStream2;
use syn::{
    Ident, Token, Type,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

/// `#[transitions(error = …?, ctx = …?)]`.
pub(crate) struct TransitionsArgs {
    /// Optional. When omitted, generated code reads the error type from the
    /// carrier's `Pipelined::Error` projection.
    pub error: Option<Type>,
    pub ctx: Ident,
}

impl Parse for TransitionsArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut error: Option<Type> = None;
        let mut ctx: Option<Ident> = None;

        let args: Punctuated<MetaKv, Token![,]> = Punctuated::parse_terminated(input)?;
        for kv in args {
            let value = kv.value.ok_or_else(|| {
                syn::Error::new(
                    kv.key.span(),
                    format!("`{}` requires a value (write `{} = …`)", kv.key, kv.key),
                )
            })?;
            match kv.key.to_string().as_str() {
                "error" => {
                    let ty: Type = syn::parse2(value)?;
                    error = Some(ty);
                }
                "ctx" => {
                    let id: Ident = syn::parse2(value)?;
                    ctx = Some(id);
                }
                other => {
                    return Err(syn::Error::new(
                        kv.key.span(),
                        format!(
                            "unknown argument `{other}` in `#[transitions(...)]`; expected `error` or `ctx`"
                        ),
                    ));
                }
            }
        }

        let ctx = ctx.unwrap_or_else(|| Ident::new("ctx", proc_macro2::Span::call_site()));

        Ok(TransitionsArgs { error, ctx })
    }
}

/// `#[transition(into = …, breakpoint?)]`.
///
/// `into` is required, but the missing-`into` check is deferred to the
/// caller so it can emit a `Diagnostic` with a help line — the `Parse`
/// trait's `syn::Result` return type can't carry sub-diagnostics.
pub(super) struct TransitionArgs {
    pub into: Option<Type>,
    /// True when the user wrote `#[transition(..., breakpoint)]` — forces
    /// an `async fn` body to resolve to a `Result<Resolved, E>` at the
    /// call site rather than lifting the chain to `InFlight`.
    pub breakpoint: bool,
    pub breakpoint_span: proc_macro2::Span,
}

impl Parse for TransitionArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut into: Option<Type> = None;
        let mut breakpoint = false;
        let mut breakpoint_span = proc_macro2::Span::call_site();

        let args: Punctuated<MetaKv, Token![,]> = Punctuated::parse_terminated(input)?;
        for kv in args {
            match kv.key.to_string().as_str() {
                "into" => {
                    let value = kv.value.ok_or_else(|| {
                        syn::Error::new(
                            kv.key.span(),
                            "`into` requires a type (write `into = NextState`)",
                        )
                    })?;
                    let ty: Type = syn::parse2(value)?;
                    into = Some(ty);
                }
                "breakpoint" => {
                    if kv.value.is_some() {
                        return Err(syn::Error::new(
                            kv.key.span(),
                            "`breakpoint` is a flag, not a key=value (write `breakpoint`, not `breakpoint = …`)",
                        ));
                    }
                    breakpoint = true;
                    breakpoint_span = kv.key.span();
                }
                other => {
                    return Err(syn::Error::new(
                        kv.key.span(),
                        format!(
                            "unknown argument `{other}` in `#[transition(...)]`; expected `into` or `breakpoint`"
                        ),
                    ));
                }
            }
        }

        Ok(TransitionArgs {
            into,
            breakpoint,
            breakpoint_span,
        })
    }
}

/// Either `key = value` (where `value` is everything up to the next
/// top-level comma) or a bare `key` flag. Tracks angle-bracket depth so
/// generic args like `Foo<A, B>` are kept intact when consuming a value.
struct MetaKv {
    key: Ident,
    value: Option<TokenStream2>,
}

impl Parse for MetaKv {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        if !input.peek(Token![=]) {
            return Ok(MetaKv { key, value: None });
        }
        input.parse::<Token![=]>()?;
        let mut value = TokenStream2::new();
        let mut angle_depth = 0i32;
        while !input.is_empty() {
            if angle_depth == 0 && input.peek(Token![,]) {
                break;
            }
            let tt: proc_macro2::TokenTree = input.parse()?;
            if let proc_macro2::TokenTree::Punct(p) = &tt {
                match p.as_char() {
                    '<' => angle_depth += 1,
                    '>' => angle_depth = (angle_depth - 1).max(0),
                    _ => {}
                }
            }
            value.extend(std::iter::once(tt));
        }
        Ok(MetaKv {
            key,
            value: Some(value),
        })
    }
}
