//! `#[derive(TypestateFactory)]` — typestate accumulator with optional
//! Pipeline integration.
//!
//! Layout:
//!
//! - [`container`] / [`field`] — attribute parsing.
//! - [`codegen`] — per-piece generators (bag struct, setters, getters,
//!   finalize, drop, etc).
//! - [`pipeline_pair`] — shared scaffolding for pipeline-arm method pairs.
//! - [`flags`] / [`parse_util`] — small helpers shared across generators.

mod carrier;
mod codegen;
mod container;
mod field;
mod flags;
mod parse_util;
mod pipeline_pair;
mod spec;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, spanned::Spanned};

use crate::{diag::MacroResult, prefix::facade_path, typestate_factory::carrier::Carrier};

use self::{
    codegen::{
        gen_bag_struct, gen_default_helper, gen_drop_impl, gen_empty_alias, gen_finalize_async,
        gen_finalize_sync, gen_getter, gen_new_impl, gen_overrider, gen_ready_trait, gen_remover,
        gen_setter,
    },
    container::Container,
    field::FieldInfo,
};

pub(crate) fn expand(input: DeriveInput) -> MacroResult<TokenStream2> {
    let original_name = &input.ident;
    let vis = &input.vis;

    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(&input, "TypestateFactory requires a struct").into());
    };
    let Fields::Named(fields) = &data.fields else {
        return Err(
            syn::Error::new_spanned(&input, "TypestateFactory requires named fields").into(),
        );
    };
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            input.generics.span(),
            "TypestateFactory does not yet support generic structs",
        )
        .into());
    }

    let container = Container::parse(&input)?;
    let bag = &container.bag_name;
    let fields: Vec<FieldInfo> = fields
        .named
        .iter()
        .map(FieldInfo::parse)
        .collect::<MacroResult<_>>()?;

    if fields.iter().any(|f| f.fallible) && container.error.is_none() {
        return Err(syn::Error::new(
            input.span(),
            "fallible setters require `#[factory(error = <Type>)]`",
        )
        .into());
    }

    let prefix = facade_path();
    let no_unsafe = container.no_unsafe;

    let bag_struct = gen_bag_struct(bag, vis, &fields, no_unsafe, &prefix);
    let new_impl = gen_new_impl(bag, vis, &fields, no_unsafe, &prefix);
    // Internal fields skip setter / remover / overrider / default-helper
    // emission — they are set positionally in `new(…)` and locked from then
    // on. Getters still emit (always callable; the field is unconditionally
    // present on every bag shape).
    let setters = fields
        .iter()
        .filter(|f| !f.internal)
        .map(|f| {
            gen_setter(
                bag,
                &fields,
                f,
                container.error.as_ref(),
                Carrier::Standalone,
                no_unsafe,
                &prefix,
            )
        })
        .collect::<TokenStream2>();
    let getters = fields
        .iter()
        .map(|f| gen_getter(bag, &fields, f, Carrier::Standalone, no_unsafe, &prefix))
        .collect::<TokenStream2>();
    let default_helpers = fields
        .iter()
        .filter(|f| !f.internal)
        .filter_map(|f| {
            gen_default_helper(bag, &fields, f, Carrier::Standalone, no_unsafe, &prefix)
        })
        .collect::<TokenStream2>();
    let removers = fields
        .iter()
        .filter(|f| f.removable && !f.internal)
        .map(|f| gen_remover(bag, &fields, f, Carrier::Standalone, no_unsafe, &prefix))
        .collect::<TokenStream2>();
    let overriders = fields
        .iter()
        .filter(|f| f.overridable && !f.internal)
        .map(|f| {
            gen_overrider(
                bag,
                &fields,
                f,
                container.error.as_ref(),
                Carrier::Standalone,
                no_unsafe,
                &prefix,
            )
        })
        .collect::<TokenStream2>();
    let drop_impl = gen_drop_impl(bag, &fields, no_unsafe, &prefix);
    let finalize_impl = gen_finalize_sync(bag, original_name, vis, &fields, no_unsafe, &prefix);
    let ready_trait = gen_ready_trait(bag, original_name, vis, &fields, no_unsafe, &prefix);
    let empty_alias = gen_empty_alias(bag, vis, &fields, &prefix);
    let finalize_async_impl = container
        .finalize_async
        .as_ref()
        .map(|spec| gen_finalize_async(bag, vis, &fields, spec, no_unsafe, &prefix))
        .unwrap_or_default();

    let mut out = quote! {
        #bag_struct
        #new_impl
        #setters
        #getters
        #default_helpers
        #removers
        #overriders
        #drop_impl
        #finalize_impl
        #ready_trait
        #empty_alias
        #finalize_async_impl
    };

    if let Some(spec) = &container.pipeline {
        let carrier = Carrier::Pipeline(spec);
        let pipe_setters = fields
            .iter()
            .filter(|f| !f.internal)
            .map(|f| {
                gen_setter(
                    bag,
                    &fields,
                    f,
                    container.error.as_ref(),
                    carrier,
                    no_unsafe,
                    &prefix,
                )
            })
            .collect::<TokenStream2>();
        let pipe_getters = fields
            .iter()
            .map(|f| gen_getter(bag, &fields, f, carrier, no_unsafe, &prefix))
            .collect::<TokenStream2>();
        let pipe_default_helpers = fields
            .iter()
            .filter(|f| !f.internal)
            .filter_map(|f| gen_default_helper(bag, &fields, f, carrier, no_unsafe, &prefix))
            .collect::<TokenStream2>();
        let pipe_removers = fields
            .iter()
            .filter(|f| f.removable && !f.internal)
            .map(|f| gen_remover(bag, &fields, f, carrier, no_unsafe, &prefix))
            .collect::<TokenStream2>();
        let pipe_overriders = fields
            .iter()
            .filter(|f| f.overridable && !f.internal)
            .map(|f| {
                gen_overrider(
                    bag,
                    &fields,
                    f,
                    container.error.as_ref(),
                    carrier,
                    no_unsafe,
                    &prefix,
                )
            })
            .collect::<TokenStream2>();

        out.extend(quote! {
            #pipe_setters
            #pipe_getters
            #pipe_default_helpers
            #pipe_removers
            #pipe_overriders
        });
    }

    Ok(out)
}
