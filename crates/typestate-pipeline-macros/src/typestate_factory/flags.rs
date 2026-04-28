use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

use crate::typestate_factory::field::FieldInfo;

#[derive(Clone, Copy)]
pub enum FlagPick {
    Yes,
    No,
}

/// Build the per-flag-generic argument list for a bag type instantiation.
///
/// Yields one token stream per non-internal field in declaration order
/// (internal fields have no flag generic). The slot for `this` is forced
/// to `Yes` or `No` per `pick`; the others forward their flag generic.
pub fn flag_args(
    all: &[FieldInfo],
    this: &Ident,
    pick: FlagPick,
    prefix: &TokenStream2,
) -> Vec<TokenStream2> {
    all.iter()
        .filter(|f| !f.internal)
        .map(|f| {
            if f.ident == *this {
                match pick {
                    FlagPick::Yes => quote!(#prefix::__private::Yes),
                    FlagPick::No => quote!(#prefix::__private::No),
                }
            } else {
                let p = &f.flag_param;
                quote!(#p)
            }
        })
        .collect()
}

fn other_satisfiable_bounds<'a>(
    all: &'a [FieldInfo],
    this: &'a Ident,
    prefix: &'a TokenStream2,
) -> impl Iterator<Item = TokenStream2> + 'a {
    all.iter()
        .filter(move |f| !f.internal && f.ident != *this)
        .map(move |f| {
            let p = &f.flag_param;
            quote!( #p: #prefix::__private::Satisfiable )
        })
}

fn other_storage_bounds<'a>(
    all: &'a [FieldInfo],
    this: &'a Ident,
    prefix: &'a TokenStream2,
) -> impl Iterator<Item = TokenStream2> + 'a {
    all.iter()
        .filter(move |f| !f.internal && f.ident != *this)
        .map(move |f| {
            let p = &f.flag_param;
            let t = &f.ty;
            quote!( #p: #prefix::__private::Storage<#t> )
        })
}

/// Per-mode flag bounds for the *other* (non-`this`, non-internal) fields:
/// `Storage<T>` in safe mode (which transitively satisfies `Satisfiable`
/// and brings the `<Flag as Storage<T>>::Out` projection into scope),
/// `Satisfiable` in unsafe mode.
pub fn other_field_bounds(
    all: &[FieldInfo],
    this: &Ident,
    no_unsafe: bool,
    prefix: &TokenStream2,
) -> Vec<TokenStream2> {
    if no_unsafe {
        other_storage_bounds(all, this, prefix).collect()
    } else {
        other_satisfiable_bounds(all, this, prefix).collect()
    }
}
