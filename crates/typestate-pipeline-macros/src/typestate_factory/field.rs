use convert_case::{Case, Casing};
use quote::format_ident;
use syn::{spanned::Spanned, Field, Ident, Type, parse_quote};

use crate::typestate_factory::parse_util::{OnlyOneSet, parse_flag};

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
    pub fn parse(field: &Field) -> syn::Result<Self> {
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
        let overridable = overridable.into_inner_optional().unwrap_or(false);
        let removable = removable.into_inner_optional().unwrap_or(false);
        let default_value = default_value.into_inner_optional();
        let explicit_name = explicit_name.into_inner_optional();
        let explicit_default_helper = explicit_default_helper.into_inner_optional();
        let setter_fn = setter_fn.into_inner_optional();
        let fallible = fallible.into_inner_optional().unwrap_or(false);
        let async_fn = async_fn.into_inner_optional().unwrap_or(false);
        let internal = internal.into_inner_optional().unwrap_or(false);
        let input_ty = input_ty.into_inner_optional();

        if explicit_default_helper.is_some() && default_value.is_none() {
            return Err(syn::Error::new(
                field.span(),
                "`default_helper = <ident>` is only valid alongside `default = …`",
            ));
        }
        if fallible && setter_fn.is_none() {
            return Err(syn::Error::new(
                field.span(),
                "`fallible` is only valid alongside `setter = <fn>`",
            ));
        }
        if async_fn && setter_fn.is_none() {
            return Err(syn::Error::new(
                field.span(),
                "`async_fn` is only valid alongside `setter = <fn>`",
            ));
        }
        if input_ty.is_some() && setter_fn.is_none() {
            return Err(syn::Error::new(
                field.span(),
                "`input = <Type>` is only valid alongside `setter = <fn>` — the transformer \
                 is what converts the input type to the field's storage type.",
            ));
        }
        // Defaults reach the setter as a plain value, so a fallible / async
        // setter can't be paired with a sync infallible default expression
        // without surprising semantics.
        if default_value.is_some() && fallible {
            return Err(syn::Error::new(
                field.span(),
                "`default` is incompatible with `fallible` — declared defaults must be \
                 infallible. Call the setter directly with your value if it needs validation.",
            ));
        }
        if default_value.is_some() && async_fn {
            return Err(syn::Error::new(
                field.span(),
                "`default` is incompatible with `async_fn` — declared defaults must be \
                 synchronous. Call the setter directly with your value if it needs async work.",
            ));
        }

        // Internal means "set at construction, locked from then on" —
        // every mutability-implying attribute below contradicts that
        // contract; reject the combinations explicitly so misuse surfaces
        // a precise error rather than a confusing setter-shape mismatch.
        if internal {
            if explicit_required == Some(false) {
                return Err(syn::Error::new(
                    field.span(),
                    "`internal` is incompatible with `optional` — internal fields are always \
                     set at construction time, so they can't be optional.",
                ));
            }
            if default_value.is_some() {
                return Err(syn::Error::new(
                    field.span(),
                    "`internal` is incompatible with `default = …` — internal fields are set \
                     at construction time, so a default has nothing to fall back to.",
                ));
            }
            if overridable {
                return Err(syn::Error::new(
                    field.span(),
                    "`internal` is incompatible with `overridable` — internal fields can't be \
                     overridden after construction.",
                ));
            }
            if removable {
                return Err(syn::Error::new(
                    field.span(),
                    "`internal` is incompatible with `removable` — internal fields can't be \
                     removed after construction.",
                ));
            }
            if setter_fn.is_some() {
                return Err(syn::Error::new(
                    field.span(),
                    "`internal` is incompatible with `setter = …` — internal fields have no \
                     setter (they're set positionally on `new(…)` instead).",
                ));
            }
            if fallible {
                return Err(syn::Error::new(
                    field.span(),
                    "`internal` is incompatible with `fallible` — internal fields have no \
                     setter to be fallible.",
                ));
            }
            if async_fn {
                return Err(syn::Error::new(
                    field.span(),
                    "`internal` is incompatible with `async_fn` — internal fields have no \
                     setter to be async.",
                ));
            }
            if input_ty.is_some() {
                return Err(syn::Error::new(
                    field.span(),
                    "`internal` is incompatible with `input = …` — internal fields have no \
                     setter, so there's no input type to override.",
                ));
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
        let default_helper_name = match (default_value.as_ref(), explicit_default_helper) {
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
