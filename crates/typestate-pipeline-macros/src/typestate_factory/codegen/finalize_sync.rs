use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Ident, Visibility};

use crate::typestate_factory::field::FieldInfo;

/// Emit the bag's inherent `finalize(self) -> <Original>` method.
///
/// Per-field bounds and read shapes vary by field role and codegen mode:
///
/// **Unsafe mode**
///
/// All field reads land in stack locals *before* any `default = …`
/// expression is evaluated, so a panicking default unwinds with every
/// already-read field as an owned local that auto-drops. (Inlining the
/// reads into the struct expression alongside defaults would leak the
/// fields *after* the panicking default — they'd still be sitting in
/// `this`'s `MaybeUninit` slots, which `ManuallyDrop` will not drop.)
///
/// - Internal: read via `unsafe { ptr::read(&this.<f>) }`.
/// - Required / optional-without-default: flag = `Satisfied`,
///   read via `unsafe { ptr::read(&this.<f>).assume_init() }`.
/// - Optional with declared default: flag = `Satisfiable`, read into
///   `Option<T>` based on `IS_SET`; the default thunk runs at the
///   `unwrap_or_else` step.
///
/// **Safe mode** allows partial moves out of `self` since the bag has no
/// manual `Drop`:
///
/// - Internal: `self.<f>` (plain `T`).
/// - Required / optional-without-default: slot pinned to concrete `Yes`,
///   read is a plain move.
/// - Optional with declared default: flag = `Storage<T>`, dispatch via
///   `Storage::finalize_or` (resolved at monomorphization).
pub fn gen_finalize_sync(
    bag: &Ident,
    original: &Ident,
    vis: &Visibility,
    fields: &[FieldInfo],
    no_unsafe: bool,
    prefix: &TokenStream2,
) -> TokenStream2 {
    let mut flag_params: Vec<TokenStream2> = Vec::new();
    let mut impl_generics: Vec<TokenStream2> = Vec::new();
    // Safe mode: in-struct field expressions (partial moves directly out
    // of `this`).
    let mut field_reads_safe: Vec<TokenStream2> = Vec::new();
    // Unsafe mode: split the work — `prelude_reads_unsafe` populates
    // stack locals with `ptr::read`s, and `field_assigns_unsafe` wires
    // those locals (or `unwrap_or_else(default)`) into the struct.
    let mut prelude_reads_unsafe: Vec<TokenStream2> = Vec::new();
    let mut field_assigns_unsafe: Vec<TokenStream2> = Vec::new();

    for f in fields {
        let n = &f.ident;
        let local = format_ident!("__tsh_finalize_{}", n);
        if f.internal {
            if no_unsafe {
                field_reads_safe.push(quote! { #n: __tsh_this.#n });
            } else {
                prelude_reads_unsafe.push(quote! {
                    // SAFETY: internal fields are stored as plain `T` and
                    // are always present.
                    let #local = unsafe { #prefix::__private::ptr::read(&__tsh_this.#n) };
                });
                field_assigns_unsafe.push(quote! { #n: #local });
            }
            continue;
        }

        let p = &f.flag_param;
        let t = &f.ty;
        let optional_with_default = !f.required && f.default_value.is_some();

        if optional_with_default {
            if no_unsafe {
                impl_generics.push(quote!( #p: #prefix::__private::Storage<#t> ));
            } else {
                impl_generics.push(quote!( #p: #prefix::__private::Satisfiable ));
            }
            flag_params.push(quote!( #p ));

            let default_expr = f.default_value.clone().unwrap();
            if no_unsafe {
                field_reads_safe.push(quote! {
                    #n: <#p as #prefix::__private::Storage<#t>>::finalize_or(
                        __tsh_this.#n,
                        || #default_expr,
                    )
                });
            } else {
                // Read every initialized field into a stack local *first*;
                // the default expression is the last thing evaluated. If
                // `#default_expr` panics, the locals drop on unwind so no
                // already-read field leaks.
                prelude_reads_unsafe.push(quote! {
                    let #local: ::core::option::Option<#t> =
                        if <#p as #prefix::__private::Satisfiable>::IS_SET {
                            // SAFETY: flag witnesses initialization.
                            ::core::option::Option::Some(unsafe {
                                #prefix::__private::ptr::read(&__tsh_this.#n).assume_init()
                            })
                        } else {
                            ::core::option::Option::None
                        };
                });
                field_assigns_unsafe.push(quote! {
                    #n: #local.unwrap_or_else(|| #default_expr)
                });
            }
        } else if no_unsafe {
            // Pin the slot to concrete `Yes` — no generic param, no bound.
            // The field's storage type then resolves to `T`.
            flag_params.push(quote!( #prefix::__private::Yes ));
            field_reads_safe.push(quote! { #n: __tsh_this.#n });
        } else {
            impl_generics.push(quote!( #p: #prefix::__private::Satisfied ));
            flag_params.push(quote!( #p ));
            prelude_reads_unsafe.push(quote! {
                // SAFETY: flag is `Satisfied`, so the slot is initialized.
                let #local = unsafe {
                    #prefix::__private::ptr::read(&__tsh_this.#n).assume_init()
                };
            });
            field_assigns_unsafe.push(quote! { #n: #local });
        }
    }

    let body = if no_unsafe {
        quote! {
            let __tsh_this = self;
            #original {
                #( #field_reads_safe, )*
            }
        }
    } else {
        quote! {
            let __tsh_this = #prefix::__private::ManuallyDrop::new(self);
            #( #prelude_reads_unsafe )*
            #original {
                #( #field_assigns_unsafe, )*
            }
        }
    };

    quote! {
        impl< #( #impl_generics ),* > #bag< #( #flag_params ),* > {
            /// Consume the bag and assemble the finalized struct.
            ///
            /// Required fields' flags must be `Yes`. Optional fields with a
            /// declared default may finalize in either state; other
            /// optional fields must be `Yes`. Internal fields are always
            /// present.
            // The macro emits stack locals named `__tsh_finalize_<field>`;
            // when a user field starts with `_` the concatenation breaks
            // the snake-case lint. The emitted bindings are internal.
            #[allow(non_snake_case)]
            #vis fn finalize(self) -> #original {
                #body
            }
        }
    }
}
