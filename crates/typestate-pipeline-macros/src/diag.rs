//! Unified error type for macro expansion. Bridges `syn::Error` (parse-time
//! errors from `syn` itself) and `proc_macro2_diagnostics::Diagnostic`
//! (validation-time errors with structured `help:` / `note:` sub-diagnostics).
//!
//! On the failure path the macro entry point converts to a `TokenStream` of
//! `compile_error!(…)` invocations — `syn::Error` via `to_compile_error()`
//! and `Diagnostic` via `emit_as_item_tokens()`.

use proc_macro2::TokenStream as TokenStream2;
use proc_macro2_diagnostics::Diagnostic;

pub enum MacroError {
    Syn(syn::Error),
    Diag(Diagnostic),
}

impl MacroError {
    pub fn into_compile_error(self) -> TokenStream2 {
        match self {
            Self::Syn(e) => e.to_compile_error(),
            Self::Diag(d) => d.emit_as_item_tokens(),
        }
    }
}

impl From<syn::Error> for MacroError {
    fn from(e: syn::Error) -> Self {
        Self::Syn(e)
    }
}

impl From<Diagnostic> for MacroError {
    fn from(d: Diagnostic) -> Self {
        Self::Diag(d)
    }
}

pub type MacroResult<T> = Result<T, MacroError>;
