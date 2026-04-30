use convert_case::{Case, Casing};
use proc_macro2_diagnostics::SpanDiagnosticExt;
use quote::format_ident;
use syn::{Field, Ident, Path, Type, parse_quote, spanned::Spanned};

use crate::{
    diag::MacroResult,
    typestate_factory::parse_util::{OnlyOneSet, parse_flag},
};

fn flag_value(slot: &Option<(bool, Path)>) -> bool {
    slot.as_ref().map(|(v, _)| *v).unwrap_or(false)
}

pub struct FieldInfo {
    /// Field identifier (e.g. `name`).
    pub ident: Ident,
    /// Field type (e.g. `String`).
    pub ty: Type,
    /// Generic parameter name on the bag (e.g. `NameFlag`).
    pub flag_param: Ident,
    /// Setter method name.
    pub setter_name: Ident,
    /// Default-helper method name (`<field>_default`), only when default present.
    pub default_helper_name: Option<Ident>,
    /// Default expression, when provided.
    pub default_value: Option<syn::Expr>,
    /// `true` for required fields.
    pub required: bool,
    /// `true` if `#[field(removable)]` — generates `drop_<field>`.
    pub removable: bool,
    /// `true` if `#[field(overridable)]` — generates `override_<field>` on Yes.
    pub overridable: bool,
    /// Custom setter transformer fn (called as `<fn>(val)` inside the setter).
    pub setter_fn: Option<Ident>,
    /// `true` if the custom transformer returns `Result<FieldType, Error>`.
    pub fallible: bool,
    /// `true` if the custom transformer is `async fn`.
    pub async_fn: bool,
    /// `true` if `#[field(internal)]`. Internal fields are set positionally
    /// at `new(…)` and locked from then on: no flag generic, no setter /
    /// overrider / remover / default-helper, unconditional getter, stored
    /// as plain `T`. Combined with any mutability-implying attribute, the
    /// derive errors at parse time.
    pub internal: bool,
    /// Override for the setter's input parameter type. Only valid alongside
    /// `setter = …`, since the transformer is what bridges `input → ty`.
    pub input_ty: Option<Type>,
}

impl FieldInfo {
    pub fn parse(field: &Field) -> MacroResult<Self> {
        let ident = field
            .ident
            .clone()
            .ok_or_else(|| syn::Error::new_spanned(field, "field must have a name"))?;
        let ty = field.ty.clone();
        let flag_param = format_ident!("{}Flag", ident.to_string().to_case(Case::Pascal));

        let mut required: OnlyOneSet<bool> = OnlyOneSet::default();
        let mut overridable: OnlyOneSet<bool> = OnlyOneSet::default();
        let mut removable: OnlyOneSet<bool> = OnlyOneSet::default();
        let mut default_value: OnlyOneSet<syn::Expr> = OnlyOneSet::default();
        let mut explicit_name: OnlyOneSet<Ident> = OnlyOneSet::default();
        let mut explicit_default_helper: OnlyOneSet<Ident> = OnlyOneSet::default();
        let mut setter_fn: OnlyOneSet<Ident> = OnlyOneSet::default();
        let mut fallible: OnlyOneSet<bool> = OnlyOneSet::default();
        let mut async_fn: OnlyOneSet<bool> = OnlyOneSet::default();
        let mut internal: OnlyOneSet<bool> = OnlyOneSet::default();
        let mut input_ty: OnlyOneSet<Type> = OnlyOneSet::default();

        for attr in &field.attrs {
            if !attr.path().is_ident("field") {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("required") {
                    required.set(parse_flag(&meta)?, &meta.path)?;
                } else if meta.path.is_ident("optional") {
                    required.set(!parse_flag(&meta)?, &meta.path)?;
                } else if meta.path.is_ident("default") {
                    let value: syn::Expr = if meta.input.peek(syn::Token![=]) {
                        meta.value()?.parse()?
                    } else {
                        parse_quote!(::core::default::Default::default())
                    };
                    default_value.set(value, &meta.path)?;
                } else if meta.path.is_ident("overridable") {
                    overridable.set(parse_flag(&meta)?, &meta.path)?;
                } else if meta.path.is_ident("removable") {
                    removable.set(parse_flag(&meta)?, &meta.path)?;
                } else if meta.path.is_ident("name") {
                    let id: Ident = meta.value()?.parse()?;
                    explicit_name.set(id, &meta.path)?;
                } else if meta.path.is_ident("default_helper") {
                    let id: Ident = meta.value()?.parse()?;
                    explicit_default_helper.set(id, &meta.path)?;
                } else if meta.path.is_ident("setter") {
                    let id: Ident = meta.value()?.parse()?;
                    setter_fn.set(id, &meta.path)?;
                } else if meta.path.is_ident("fallible") {
                    fallible.set(parse_flag(&meta)?, &meta.path)?;
                } else if meta.path.is_ident("async_fn") {
                    async_fn.set(parse_flag(&meta)?, &meta.path)?;
                } else if meta.path.is_ident("internal") {
                    internal.set(parse_flag(&meta)?, &meta.path)?;
                } else if meta.path.is_ident("input") {
                    let ty: Type = meta.value()?.parse()?;
                    input_ty.set(ty, &meta.path)?;
                } else {
                    return Err(syn::Error::new_spanned(
                        &meta.path,
                        "unknown field attribute (expected `required`, `optional`, `default`, \
                         `overridable`, `removable`, `name`, `default_helper`, `setter`, \
                         `fallible`, `async_fn`, `internal`, or `input`)",
                    ));
                }
                Ok(())
            })?;
        }

        let explicit_required = required.into_inner_optional();
        let overridable_with_path = overridable.into_inner_with_path();
        let overridable = flag_value(&overridable_with_path);
        let removable_with_path = removable.into_inner_with_path();
        let removable = flag_value(&removable_with_path);
        let default_value_with_path = default_value.into_inner_with_path();
        let default_value = default_value_with_path.as_ref().map(|(v, _)| v.clone());
        let default_path = default_value_with_path.as_ref().map(|(_, p)| p);
        let explicit_name = explicit_name.into_inner_optional();
        let explicit_default_helper = explicit_default_helper.into_inner_with_path();
        let explicit_default_helper_value =
            explicit_default_helper.as_ref().map(|(v, _)| v.clone());
        let setter_fn_with_path = setter_fn.into_inner_with_path();
        let setter_fn = setter_fn_with_path.as_ref().map(|(v, _)| v.clone());
        let setter_path = setter_fn_with_path.as_ref().map(|(_, p)| p);
        let fallible_with_path = fallible.into_inner_with_path();
        let fallible = flag_value(&fallible_with_path);
        let async_fn_with_path = async_fn.into_inner_with_path();
        let async_fn = flag_value(&async_fn_with_path);
        let internal_with_path = internal.into_inner_with_path();
        let internal = flag_value(&internal_with_path);
        let internal_path = internal_with_path.as_ref().map(|(_, p)| p);
        let input_ty_with_path = input_ty.into_inner_with_path();
        let input_ty = input_ty_with_path.as_ref().map(|(v, _)| v.clone());
        let input_path = input_ty_with_path.as_ref().map(|(_, p)| p);

        if let Some((_, helper_path)) = explicit_default_helper.as_ref()
            && default_value.is_none()
        {
            return Err(helper_path
                .span()
                .error("`default_helper = <ident>` is only valid alongside `default = …`")
                .help("add `default = <expr>` to declare the default this helper overrides, or remove `default_helper`")
                .into());
        }
        if let Some((true, fallible_path)) = fallible_with_path.as_ref()
            && setter_fn.is_none()
        {
            return Err(fallible_path
                .span()
                .error("`fallible` is only valid alongside `setter = <fn>`")
                .help("`fallible` describes the setter transformer's return type — add `setter = <fn>` (the fn returning `Result<FieldType, Error>`) or remove `fallible`")
                .into());
        }
        if let Some((true, async_path)) = async_fn_with_path.as_ref()
            && setter_fn.is_none()
        {
            return Err(async_path
                .span()
                .error("`async_fn` is only valid alongside `setter = <fn>`")
                .help("`async_fn` describes the setter transformer's shape — add `setter = <fn>` (the `async fn`) or remove `async_fn`")
                .into());
        }
        if let Some(input_path) = input_path
            && setter_fn.is_none()
        {
            return Err(input_path
                .span()
                .error("`input = <Type>` is only valid alongside `setter = <fn>`")
                .note("the transformer is what converts the input type to the field's storage type")
                .help("add `setter = <fn>` (the fn that bridges `input → ty`) or remove `input`")
                .into());
        }
        // Defaults reach the setter as a plain value, so a fallible / async
        // setter can't be paired with a sync infallible default expression
        // without surprising semantics.
        if let Some(default_path) = default_path
            && fallible
        {
            return Err(default_path
                .span()
                .error("`default` is incompatible with `fallible`")
                .note("declared defaults must be infallible — `fallible` setters return `Result`, but a default expression cannot")
                .help("call the setter directly with your value if it needs validation, instead of declaring a default")
                .into());
        }
        if let Some(default_path) = default_path
            && let Some((true, async_path)) = async_fn_with_path.as_ref()
        {
            return Err(default_path
                .span()
                .error("`default` is incompatible with `async_fn`")
                .span_note(async_path.span(), "`async_fn` declared here")
                .note("declared defaults must be synchronous — `async fn` setters return a future, but a default expression is evaluated eagerly")
                .help("call the setter directly with your value if it needs async work, instead of declaring a default")
                .into());
        }

        // Internal means "set at construction, locked from then on" —
        // every mutability-implying attribute below contradicts that
        // contract; reject the combinations explicitly so misuse surfaces
        // a precise error rather than a confusing setter-shape mismatch.
        if let Some(internal_path) = internal_path {
            let here = || -> proc_macro2::Span { internal_path.span() };
            if explicit_required == Some(false) {
                return Err(here()
                    .error("`internal` is incompatible with `optional`")
                    .note("internal fields are always set at construction time, so they can't be optional")
                    .help("remove `internal` if the field should be optional, or remove `optional` if it must be set at construction")
                    .into());
            }
            if let Some(default_path) = default_path {
                return Err(default_path
                    .span()
                    .error("`internal` is incompatible with `default = …`")
                    .span_note(here(), "`internal` declared here")
                    .note("internal fields are set at construction time, so a default has nothing to fall back to")
                    .help("remove `internal` to keep the default, or remove `default` and pass the value to `new(…)` positionally")
                    .into());
            }
            if let Some((true, overridable_path)) = overridable_with_path.as_ref() {
                return Err(overridable_path
                    .span()
                    .error("`internal` is incompatible with `overridable`")
                    .span_note(here(), "`internal` declared here")
                    .note("internal fields can't be overridden after construction")
                    .help("remove `internal` to allow override, or remove `overridable`")
                    .into());
            }
            if let Some((true, removable_path)) = removable_with_path.as_ref() {
                return Err(removable_path
                    .span()
                    .error("`internal` is incompatible with `removable`")
                    .span_note(here(), "`internal` declared here")
                    .note("internal fields can't be removed after construction")
                    .help("remove `internal` to allow `drop_<field>`, or remove `removable`")
                    .into());
            }
            if let Some(setter_path) = setter_path {
                return Err(setter_path
                    .span()
                    .error("`internal` is incompatible with `setter = …`")
                    .span_note(here(), "`internal` declared here")
                    .note("internal fields have no setter — they're set positionally on `new(…)`")
                    .help("remove `internal` to expose a setter, or remove `setter = …`")
                    .into());
            }
            if let Some((true, fallible_path)) = fallible_with_path.as_ref() {
                return Err(fallible_path
                    .span()
                    .error("`internal` is incompatible with `fallible`")
                    .span_note(here(), "`internal` declared here")
                    .note("internal fields have no setter to be fallible")
                    .help("remove `internal` to have a fallible setter, or remove `fallible`")
                    .into());
            }
            if let Some((true, async_path)) = async_fn_with_path.as_ref() {
                return Err(async_path
                    .span()
                    .error("`internal` is incompatible with `async_fn`")
                    .span_note(here(), "`internal` declared here")
                    .note("internal fields have no setter to be async")
                    .help("remove `internal` to have an async setter, or remove `async_fn`")
                    .into());
            }
            if let Some(input_path) = input_path {
                return Err(input_path
                    .span()
                    .error("`internal` is incompatible with `input = …`")
                    .span_note(here(), "`internal` declared here")
                    .note("internal fields have no setter, so there's no input type to override")
                    .help("remove `internal` to expose a setter, or remove `input`")
                    .into());
            }
        }

        let required = explicit_required.unwrap_or(default_value.is_none());

        // Naming convention: required → bare field name (`.name(val)`),
        // optional → `with_<field>` (`.with_age(val)`).
        let setter_name = explicit_name.unwrap_or_else(|| {
            if required {
                ident.clone()
            } else {
                format_ident!("with_{}", ident)
            }
        });
        let default_helper_name = match (default_value.as_ref(), explicit_default_helper_value) {
            (None, _) => None,
            (Some(_), Some(custom)) => Some(custom),
            (Some(_), None) => Some(format_ident!("{}_default", ident)),
        };

        Ok(FieldInfo {
            ident,
            ty,
            flag_param,
            setter_name,
            default_helper_name,
            default_value,
            required,
            removable,
            overridable,
            setter_fn,
            fallible,
            async_fn,
            internal,
            input_ty,
        })
    }
}
