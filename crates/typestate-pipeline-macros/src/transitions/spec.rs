use proc_macro2::Span;
use proc_macro2_diagnostics::SpanDiagnosticExt;
use syn::{ImplItemFn, ReturnType, Type};

use crate::diag::{MacroError, MacroResult};

use super::{args::TransitionArgs, self_ty::is_result_type};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum BodyShape {
    SyncInfallible,
    SyncFallible,
    AsyncDeferred,
    AsyncBreakpoint,
}

pub(super) struct TransitionSpec {
    pub method: ImplItemFn,
    pub into: Type,
    pub shape: BodyShape,
}

impl TransitionSpec {
    pub(super) fn from_fn(
        method: ImplItemFn,
        args: TransitionArgs,
        attr_span: Span,
    ) -> MacroResult<Self> {
        let into = args.into.ok_or_else(|| -> MacroError {
            attr_span
                .error("`into = <Type>` is required in `#[transition(...)]`")
                .help("declare the destination state, e.g. `#[transition(into = NextState)]`")
                .into()
        })?;

        let is_async = method.sig.asyncness.is_some();
        let returns_result = matches!(
            &method.sig.output,
            ReturnType::Type(_, ty) if is_result_type(ty)
        );

        let shape = match (is_async, args.breakpoint, returns_result) {
            (true, false, _) => BodyShape::AsyncDeferred,
            (true, true, _) => BodyShape::AsyncBreakpoint,
            (false, true, _) => {
                return Err(args
                    .breakpoint_span
                    .error("`breakpoint` is only valid on `async fn` transitions")
                    .span_note(method.sig.fn_token.span, "this transition is sync")
                    .help("remove `breakpoint` (sync transitions resolve immediately) or make the transition `async fn`")
                    .into());
            }
            (false, false, true) => BodyShape::SyncFallible,
            (false, false, false) => BodyShape::SyncInfallible,
        };

        Ok(TransitionSpec {
            method,
            into,
            shape,
        })
    }
}
