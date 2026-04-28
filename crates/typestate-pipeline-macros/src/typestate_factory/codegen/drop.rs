use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Ident;

use crate::typestate_factory::field::FieldInfo;

/// Emit the bag's `Drop` impl — only in unsafe mode.
///
/// Unsafe mode stores each non-internal field as `MaybeUninit<T>`, which
/// does not drop on regular drop, so we read each set field out into an
/// owned `Option<T>` stack guard. The guards drop in reverse declaration
/// order via Rust's auto-drop, which has cleanup-on-panic semantics: a
/// panicking `T::drop` on one field still lets the remaining guards drop.
/// (A naive sequence of `assume_init_drop` calls would short-circuit at
/// the first panic and leak the rest, since `MaybeUninit` slots have no
/// auto-drop fallback.)
///
/// Safe mode stores each field as `<Flag as Storage<T>>::Out` (`T` when
/// set, `()` when unset). Both auto-drop correctly, and emitting a manual
/// `Drop` would block the partial moves the safe-mode setters / removers /
/// overriders rely on. Returns an empty token stream in safe mode.
pub fn gen_drop_impl(
    bag: &Ident,
    fields: &[FieldInfo],
    no_unsafe: bool,
    prefix: &TokenStream2,
) -> TokenStream2 {
    if no_unsafe {
        return TokenStream2::new();
    }
    let flag_params: Vec<_> = fields
        .iter()
        .filter(|f| !f.internal)
        .map(|f| &f.flag_param)
        .collect();
    if flag_params.is_empty() {
        // All fields are internal (plain `T`), so auto-drop already does
        // the right thing. Emitting an empty `Drop` impl would just block
        // partial moves elsewhere with no benefit.
        return TokenStream2::new();
    }
    let bounds = flag_params
        .iter()
        .map(|p| quote!( #p: #prefix::__private::Satisfiable ));
    let field_drops = fields.iter().filter(|f| !f.internal).map(|f| {
        let n = &f.ident;
        let p = &f.flag_param;
        let g = format_ident!("__tsh_guard_{}", n);
        quote! {
            // Read the set field out into an owned stack temp so each
            // destructor runs in its own auto-drop scope. If `T::drop`
            // panics, the remaining guards still drop on unwind.
            let #g = if <#p as #prefix::__private::Satisfiable>::IS_SET {
                // SAFETY: the flag witnesses that this field was written.
                ::core::option::Option::Some(unsafe { self.#n.assume_init_read() })
            } else {
                ::core::option::Option::None
            };
        }
    });

    quote! {
        impl< #( #bounds ),* > ::core::ops::Drop for #bag< #( #flag_params ),* > {
            // Same rationale as `gen_finalize_sync`: emitted guards are
            // named `__tsh_guard_<field>`, which trips snake-case for
            // user fields starting with `_`.
            #[allow(non_snake_case)]
            fn drop(&mut self) {
                #( #field_drops )*
            }
        }
    }
}
