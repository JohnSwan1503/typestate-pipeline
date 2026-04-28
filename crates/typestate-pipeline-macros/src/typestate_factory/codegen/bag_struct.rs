use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, Visibility};

use crate::typestate_factory::field::FieldInfo;

/// Emit the bag struct.
///
/// Per-mode shape for non-internal fields:
///
/// | mode      | flag bound       | field type                                | drop                    |
/// |-----------|------------------|-------------------------------------------|-------------------------|
/// | unsafe    | `Satisfiable`    | `MaybeUninit<T>`                          | manual (`gen_drop_impl`)|
/// | no_unsafe | `Storage<T>`     | `<Flag as Storage<T>>::Out` (`T` or `()`) | auto-derived            |
///
/// Internal fields are stored as plain `T` in both modes and have no flag
/// generic.
pub fn gen_bag_struct(
    bag: &Ident,
    vis: &Visibility,
    fields: &[FieldInfo],
    no_unsafe: bool,
    prefix: &TokenStream2,
) -> TokenStream2 {
    let flag_decls = fields.iter().filter(|f| !f.internal).map(|f| {
        let p = &f.flag_param;
        let bound = if no_unsafe {
            let t = &f.ty;
            quote!( #prefix::__private::Storage<#t> )
        } else {
            quote!( #prefix::__private::Satisfiable )
        };
        quote! {
            #p: #bound
                = #prefix::__private::No
        }
    });

    let field_decls = fields.iter().map(|f| {
        let n = &f.ident;
        let t = &f.ty;
        if f.internal {
            quote! { #n: #t }
        } else if no_unsafe {
            let p = &f.flag_param;
            quote! { #n: <#p as #prefix::__private::Storage<#t>>::Out }
        } else {
            quote! { #n: #prefix::__private::MaybeUninit<#t> }
        }
    });

    let flag_idents: Vec<_> = fields
        .iter()
        .filter(|f| !f.internal)
        .map(|f| &f.flag_param)
        .collect();

    quote! {
        // Pin the bag's layout to the default Rust representation. Any
        // future addition of `#[repr(packed)]` / `#[repr(C, …)]` would be
        // a deliberate decision rather than an accidental break of the
        // `ptr::read(&this.<f>)` alignment assumption used by setters,
        // overriders, removers, and `finalize`.
        #[repr(Rust)]
        #[allow(non_camel_case_types)]
        #vis struct #bag< #( #flag_decls ),* > {
            #( #field_decls, )*
            // Macro-internal phantom field. Prefixed with `__tsh_` so it
            // is very unlikely to collide with a user-declared field
            // name.
            //
            // The trailing comma in `( #( #flag_idents, )* )` is
            // load-bearing: with one flag it produces the singleton
            // tuple `(F,)`, not the parenthesized type `(F)` (which
            // would be `F` itself). With zero flags it produces `()`.
            // Either is a valid `PhantomData` payload; the parenthesized
            // single-type form would silently change the variance and
            // auto-trait inheritance of the bag.
            __tsh_markers: #prefix::__private::PhantomData<( #( #flag_idents, )* )>,
        }
    }
}
