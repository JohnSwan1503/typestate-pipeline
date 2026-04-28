use quote::format_ident;
use syn::{DeriveInput, Ident, Path, Type};

use super::{
    parse_util::{OnlyOneSet, parse_flag},
    spec::{FinalizeAsyncSpec, PipelineSpec},
};

/// Container attributes parsed from `#[factory(...)]`.
pub struct Container {
    pub bag_name: Ident,
    pub error: Option<Type>,
    pub pipeline: Option<PipelineSpec>,
    pub finalize_async: Option<FinalizeAsyncSpec>,
    /// Opt-in to the safe-mode codegen path (no `MaybeUninit`, no `unsafe`,
    /// no manual `Drop`). Gated on the `no_unsafe` Cargo feature.
    pub no_unsafe: bool,
}

impl Container {
    pub fn parse(input: &DeriveInput) -> syn::Result<Self> {
        let original_name = &input.ident;
        let mut bag_name: OnlyOneSet<Ident> = OnlyOneSet::default();
        let mut error: OnlyOneSet<Type> = OnlyOneSet::default();
        let mut pipeline: OnlyOneSet<PipelineSpec> = OnlyOneSet::default();
        let mut finalize_async: OnlyOneSet<FinalizeAsyncSpec> = OnlyOneSet::default();
        let mut no_unsafe: OnlyOneSet<bool> = OnlyOneSet::default();

        for attr in &input.attrs {
            if !attr.path().is_ident("factory") {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let id: Ident = meta.value()?.parse()?;
                    bag_name.set(id, &meta.path)?;
                } else if meta.path.is_ident("error") {
                    let ty: Type = meta.value()?.parse()?;
                    error.set(ty, &meta.path)?;
                } else if meta.path.is_ident("pipeline") {
                    let mut carrier: OnlyOneSet<Path> = OnlyOneSet::default();
                    meta.parse_nested_meta(|inner| {
                        if inner.path.is_ident("carrier") {
                            let p: Path = inner.value()?.parse()?;
                            carrier.set(p, &inner.path)?;
                        } else {
                            return Err(syn::Error::new_spanned(
                                &inner.path,
                                "expected `carrier = <Type>` inside `pipeline(...)`",
                            ));
                        }
                        Ok(())
                    })?;
                    let carrier = carrier.into_inner_optional().ok_or_else(|| {
                        syn::Error::new_spanned(
                            &meta.path,
                            "`pipeline(carrier = <Type>)` is required",
                        )
                    })?;
                    pipeline.set(PipelineSpec { carrier }, &meta.path)?;
                } else if meta.path.is_ident("finalize_async") {
                    let mut via: OnlyOneSet<Ident> = OnlyOneSet::default();
                    let mut into: OnlyOneSet<Type> = OnlyOneSet::default();
                    let mut err_ty: OnlyOneSet<Type> = OnlyOneSet::default();
                    meta.parse_nested_meta(|inner| {
                        if inner.path.is_ident("via") {
                            let id: Ident = inner.value()?.parse()?;
                            via.set(id, &inner.path)?;
                        } else if inner.path.is_ident("into") {
                            let ty: Type = inner.value()?.parse()?;
                            into.set(ty, &inner.path)?;
                        } else if inner.path.is_ident("error") {
                            let ty: Type = inner.value()?.parse()?;
                            err_ty.set(ty, &inner.path)?;
                        } else {
                            return Err(syn::Error::new_spanned(
                                &inner.path,
                                "expected `via`, `into`, or `error` inside `finalize_async(...)`",
                            ));
                        }
                        Ok(())
                    })?;
                    let via = via.into_inner_optional().ok_or_else(|| {
                        syn::Error::new_spanned(
                            &meta.path,
                            "`finalize_async(via = <fn>)` is required",
                        )
                    })?;
                    let into = into.into_inner_optional().ok_or_else(|| {
                        syn::Error::new_spanned(
                            &meta.path,
                            "`finalize_async(into = <Type>)` is required",
                        )
                    })?;
                    let err_ty = err_ty.into_inner_optional();
                    finalize_async.set(
                        FinalizeAsyncSpec {
                            via,
                            into,
                            error: err_ty,
                        },
                        &meta.path,
                    )?;
                } else if meta.path.is_ident("no_unsafe") {
                    // Gate per-derive opt-in on the macro crate's own
                    // Cargo feature, so a bare `#[factory(no_unsafe)]` in a
                    // crate that hasn't enabled the feature surfaces a
                    // clear error rather than silently switching codegen.
                    if !cfg!(feature = "no_unsafe") {
                        return Err(syn::Error::new_spanned(
                            &meta.path,
                            "`no_unsafe` requires the `no_unsafe` feature on the \
                             `typestate-pipeline` façade crate. Add \
                             `typestate-pipeline = { …, features = [\"no_unsafe\"] }` \
                             to your Cargo.toml.",
                        ));
                    }
                    no_unsafe.set(parse_flag(&meta)?, &meta.path)?;
                } else {
                    return Err(syn::Error::new_spanned(
                        &meta.path,
                        "unknown factory attribute (expected `name`, `error`, `pipeline`, \
                         `finalize_async`, or `no_unsafe`)",
                    ));
                }
                Ok(())
            })?;
        }

        let bag_name = bag_name
            .into_inner_optional()
            .unwrap_or_else(|| format_ident!("{}Factory", original_name));
        let error = error.into_inner_optional();
        let pipeline = pipeline.into_inner_optional();
        let finalize_async = finalize_async.into_inner_optional();
        let no_unsafe = no_unsafe.into_inner_optional().unwrap_or(false);

        Ok(Container {
            bag_name,
            error,
            pipeline,
            finalize_async,
            no_unsafe,
        })
    }
}
