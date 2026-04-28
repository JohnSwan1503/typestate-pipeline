//! Resolve the path prefix used by macro expansions to refer back to the
//! `typestate-pipeline` facade.
//!
//! Two cases need different prefixes:
//!
//! 1. External user, conventional or in-package use — `::typestate_pipeline`.
//!    The facade aliases its own crate root via `extern crate self as
//!    typestate_pipeline;`, so this absolute path resolves whether the
//!    macro was invoked from a downstream crate, an in-package integration
//!    test, or an in-package `examples/` binary.
//! 2. External user, renamed dep (`pipeline = { package = "typestate-pipeline" }`)
//!    — `::pipeline`, read from `proc_macro_crate::crate_name`.

use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::Ident;

/// Path prefix that resolves to the `typestate-pipeline` facade in the
/// caller's compilation context. Generated code emits `<prefix>::__private::Foo`
/// to reach a facade item.
pub(crate) fn facade_path() -> TokenStream2 {
    match crate_name("typestate-pipeline") {
        // In-package use (lib src, integration tests, examples) and
        // conventional downstream dep both resolve through the absolute
        // path. `extern crate self as typestate_pipeline;` in the facade's
        // lib.rs makes the absolute path work from within the lib too.
        Ok(FoundCrate::Itself) => quote!(::typestate_pipeline),
        Ok(FoundCrate::Name(name)) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(::#ident)
        }
        Err(_) => quote!(::typestate_pipeline),
    }
}
