use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    GenericArgument, GenericParam, Generics, Lifetime, Path, PathArguments, Type, TypePath,
    WhereClause, punctuated::Punctuated,
};

#[derive(Clone, Copy)]
pub(super) enum Mode {
    Resolved,
    InFlight,
}

impl Mode {
    fn as_path(&self, prefix: &TokenStream2) -> Path {
        let leaf = match self {
            Mode::Resolved => quote!(Resolved),
            Mode::InFlight => quote!(InFlight),
        };
        syn::parse2(quote! { #prefix::__private::#leaf })
            .expect("prefix + leaf must always parse as a path")
    }
}

/// Append a mode marker to the impl's mode-elided self-type.
///
/// The user writes `impl<…> Carrier<…, State>` (no `Mode` argument); each
/// generated arm emits `Carrier<…, State, Resolved>` or `… InFlight>`.
pub(super) fn pipeline_self_ty(self_ty: &Type, mode: Mode, prefix: &TokenStream2) -> Type {
    let Type::Path(tp) = self_ty else {
        unreachable!("guarded by extract_carrier_ident");
    };
    let mut tp = tp.clone();
    let last = tp
        .path
        .segments
        .last_mut()
        .expect("guarded by extract_carrier_ident");
    append_mode_arg(&mut last.arguments, mode, prefix);
    Type::Path(tp)
}

fn append_mode_arg(args: &mut PathArguments, mode: Mode, prefix: &TokenStream2) {
    let mode_ty = Type::Path(TypePath {
        qself: None,
        path: mode.as_path(prefix),
    });
    match args {
        PathArguments::AngleBracketed(ab) => {
            ab.args.push(GenericArgument::Type(mode_ty));
        }
        PathArguments::None => {
            let mut new = syn::AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: Default::default(),
                args: Punctuated::new(),
                gt_token: Default::default(),
            };
            new.args.push(GenericArgument::Type(mode_ty));
            *args = PathArguments::AngleBracketed(new);
        }
        PathArguments::Parenthesized(_) => {}
    }
}

/// First lifetime parameter on the impl block, used as the `'a` in
/// `<Self as Pipelined<'a>>::…` projections.
pub(super) fn first_lifetime(generics: &Generics) -> Option<&Lifetime> {
    generics.params.iter().find_map(|p| match p {
        GenericParam::Lifetime(lt) => Some(&lt.lifetime),
        _ => None,
    })
}

pub(super) fn merge_generics(a: &Generics, b: &Generics) -> Generics {
    let mut out = a.clone();
    for p in &b.params {
        out.params.push(p.clone());
    }
    if let Some(wb) = &b.where_clause {
        let wa = out.where_clause.get_or_insert_with(|| WhereClause {
            where_token: Default::default(),
            predicates: Punctuated::new(),
        });
        for p in &wb.predicates {
            wa.predicates.push(p.clone());
        }
    }
    out
}

pub(super) fn is_result_type(ty: &Type) -> bool {
    let Type::Path(TypePath { path, .. }) = ty else {
        return false;
    };
    let Some(last) = path.segments.last() else {
        return false;
    };
    last.ident == "Result"
}
