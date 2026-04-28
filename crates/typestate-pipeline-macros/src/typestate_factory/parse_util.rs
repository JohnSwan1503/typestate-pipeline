use syn::Path;

/// Builder-side guard that captures attribute exclusivity. Each settable
/// attribute uses one of these so that a duplicate `#[factory(name = …)]`
/// or `#[field(default = …, default = …)]` produces a clear error pointing
/// at both occurrences.
#[derive(Default)]
pub enum OnlyOneSet<T> {
    #[default]
    Unset,
    Set {
        value: T,
        path: Path,
    },
}

impl<T> OnlyOneSet<T> {
    pub fn into_inner_optional(self) -> Option<T> {
        match self {
            OnlyOneSet::Unset => None,
            OnlyOneSet::Set { value, .. } => Some(value),
        }
    }

    pub fn set<U: Into<T>>(&mut self, value: U, path: &Path) -> syn::Result<()> {
        let value = value.into();
        match self {
            OnlyOneSet::Unset => {
                *self = OnlyOneSet::Set {
                    value,
                    path: path.clone(),
                };
                Ok(())
            }
            OnlyOneSet::Set {
                path: existing_path,
                ..
            } => {
                let mut err = syn::Error::new_spanned(path, "attribute already set");
                err.combine(syn::Error::new_spanned(existing_path, "first set here"));
                Err(err)
            }
        }
    }
}

/// Parse a boolean flag attribute like `#[field(required)]` (implicit true)
/// or `#[field(required = false)]` (explicit value).
pub fn parse_flag(meta: &syn::meta::ParseNestedMeta) -> syn::Result<bool> {
    if meta.input.peek(syn::Token![=]) {
        let value = meta.value()?;
        let lit: syn::LitBool = value.parse()?;
        Ok(lit.value)
    } else {
        Ok(true)
    }
}
