use syn::{ImplItemFn, ReturnType};

use super::{
    args::TransitionArgs,
    self_ty::is_result_type,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum BodyShape {
    SyncInfallible,
    SyncFallible,
    AsyncDeferred,
    AsyncBreakpoint,
}

pub(super) struct TransitionSpec {
    pub method: ImplItemFn,
    pub args: TransitionArgs,
    pub shape: BodyShape,
}

impl TransitionSpec {
    pub(super) fn from_fn(method: ImplItemFn, args: TransitionArgs) -> syn::Result<Self> {
        let is_async = method.sig.asyncness.is_some();
        let returns_result = matches!(
            &method.sig.output,
            ReturnType::Type(_, ty) if is_result_type(ty)
        );

        let shape = match (is_async, args.deferred, returns_result) {
            (true, None | Some(true), _) => BodyShape::AsyncDeferred,
            (true, Some(false), _) => BodyShape::AsyncBreakpoint,
            (false, Some(_), _) => {
                return Err(syn::Error::new(
                    args.deferred_span,
                    "`deferred` is only valid on `async fn` transitions",
                ));
            }
            (false, None, true) => BodyShape::SyncFallible,
            (false, None, false) => BodyShape::SyncInfallible,
        };

        Ok(TransitionSpec {
            method,
            args,
            shape,
        })
    }
}
