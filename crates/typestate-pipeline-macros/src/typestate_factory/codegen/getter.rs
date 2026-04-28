use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

use crate::typestate_factory::{carrier::Carrier, field::FieldInfo};

/// Emit a getter for one field.
///
/// - **Internal fields** carry no flag generic and are unconditionally
///   present, so the standalone getter is callable on every bag and the
///   pipeline-arm getter is callable on every Resolved carrier.
/// - **Non-internal fields** are gated on `<this>Flag = Yes` in the bag's
///   instantiation; the other non-internal flags pick up a per-mode bound
///   (`Storage<T>` in safe mode, `Satisfiable` in unsafe mode).
///
/// Pipeline-arm getters are Resolved-only — an `InFlight` carrier holds a
/// pending future, not a resolved bag, so there's nothing to read
/// synchronously.
pub fn gen_getter(
    bag: &Ident,
    all: &[FieldInfo],
    this: &FieldInfo,
    carrier: Carrier<'_>,
    no_unsafe: bool,
    prefix: &TokenStream2,
) -> TokenStream2 {
    let getter_name = &this.ident;
    let field_ty = &this.ty;
    let field_ident = &this.ident;

    let bag_flag_args: Vec<TokenStream2> = all
        .iter()
        .filter(|f| !f.internal)
        .map(|f| {
            if !this.internal && f.ident == this.ident {
                quote!(#prefix::__private::Yes)
            } else {
                let p = &f.flag_param;
                quote!(#p)
            }
        })
        .collect();

    let bound_fields: Vec<&FieldInfo> = all
        .iter()
        .filter(|f| !f.internal && (this.internal || f.ident != this.ident))
        .collect();
    let bag_flag_bounds: Vec<TokenStream2> = bound_fields
        .iter()
        .map(|f| {
            let p = &f.flag_param;
            if no_unsafe {
                let t = &f.ty;
                quote!( #p: #prefix::__private::Storage<#t> )
            } else {
                quote!( #p: #prefix::__private::Satisfiable )
            }
        })
        .collect();

    match carrier {
        Carrier::Standalone => {
            // Internal fields are always plain `T`. Safe-mode non-internal
            // fields resolve to `T` because the impl signature pins the flag
            // to `Yes` and `<Yes as Storage<T>>::Out = T`. Both → plain ref.
            let body = if this.internal || no_unsafe {
                quote!( &self.#field_ident )
            } else {
                quote! {
                    // SAFETY: the type-level flag witnesses that this field
                    // was written via the corresponding setter.
                    unsafe { self.#field_ident.assume_init_ref() }
                }
            };
            let doc = if this.internal {
                quote!( #[doc = "Borrow the internal field. Always callable."] )
            } else {
                quote!( #[doc = "Borrow the field. Available on bags where the field is set."] )
            };
            quote! {
                impl< #( #bag_flag_bounds ),* > #bag< #( #bag_flag_args ),* > {
                    #doc
                    #[inline]
                    pub fn #getter_name(&self) -> &#field_ty {
                        #body
                    }
                }
            }
        }
        Carrier::Pipeline(spec) => {
            // The carrier's `Pipelined::Resolved<NS>` GAT requires `NS: 'a`,
            // so each flag generic must outlive `'a` (Yes/No are 'static so
            // this is satisfied trivially at every concrete callsite).
            let carrier_path = &spec.carrier;
            let bag_flag_bounds_a: Vec<TokenStream2> = bound_fields
                .iter()
                .map(|f| {
                    let p = &f.flag_param;
                    if no_unsafe {
                        let t = &f.ty;
                        quote!( #p: #prefix::__private::Storage<#t> + 'a )
                    } else {
                        quote!( #p: #prefix::__private::Satisfiable + 'a )
                    }
                })
                .collect();
            let doc = if this.internal {
                quote!( #[doc = "Borrow the internal field on the resolved pipeline carrier."] )
            } else {
                quote!( #[doc = "Borrow the field on the resolved pipeline carrier. Available on shapes where the field is set."] )
            };
            quote! {
                impl<'a, #( #bag_flag_bounds_a ),* >
                    #carrier_path<'a, #bag< #( #bag_flag_args ),* >, #prefix::__private::Resolved>
                {
                    #doc
                    #[inline]
                    pub fn #getter_name(&self) -> &#field_ty {
                        self.0.state().#getter_name()
                    }
                }
            }
        }
    }
}
