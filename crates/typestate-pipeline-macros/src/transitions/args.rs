use proc_macro2::TokenStream as TokenStream2;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, LitBool, Token, Type,
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
            match kv.key.to_string().as_str() {
                "error" => {
                    let ty: Type = syn::parse2(kv.value)?;
                    error = Some(ty);
                }
                "ctx" => {
                    let id: Ident = syn::parse2(kv.value)?;
                    ctx = Some(id);
                }
                other => {
                    return Err(syn::Error::new(
                        kv.key.span(),
                        format!("unknown argument `{other}` in `#[transitions(...)]`; expected `error` or `ctx`"),
                    ));
                }
            }
        }

        let ctx = ctx.unwrap_or_else(|| Ident::new("ctx", proc_macro2::Span::call_site()));

        Ok(TransitionsArgs { error, ctx })
    }
}

/// `#[transition(into = …, deferred = …?)]`.
pub(super) struct TransitionArgs {
    pub into: Type,
    pub deferred: Option<bool>,
    pub deferred_span: proc_macro2::Span,
}

impl Parse for TransitionArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut into: Option<Type> = None;
        let mut deferred: Option<bool> = None;
        let mut deferred_span = proc_macro2::Span::call_site();

        let args: Punctuated<MetaKv, Token![,]> = Punctuated::parse_terminated(input)?;
        for kv in args {
            match kv.key.to_string().as_str() {
                "into" => {
                    let ty: Type = syn::parse2(kv.value)?;
                    into = Some(ty);
                }
                "deferred" => {
                    deferred_span = kv.key.span();
                    let lit: LitBool = syn::parse2(kv.value)?;
                    deferred = Some(lit.value);
                }
                other => {
                    return Err(syn::Error::new(
                        kv.key.span(),
                        format!("unknown argument `{other}` in `#[transition(...)]`; expected `into` or `deferred`"),
                    ));
                }
            }
        }

        let into = into.ok_or_else(|| {
            syn::Error::new(input.span(), "`into = <Type>` is required in `#[transition(...)]`")
        })?;

        Ok(TransitionArgs {
            into,
            deferred,
            deferred_span,
        })
    }
}

/// `key = value` pair where `value` is everything up to the next top-level
/// comma. Tracks angle-bracket depth so generic args like `Foo<A, B>` are
/// kept intact.
struct MetaKv {
    key: Ident,
    value: TokenStream2,
}

impl Parse for MetaKv {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
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
        Ok(MetaKv { key, value })
    }
}
