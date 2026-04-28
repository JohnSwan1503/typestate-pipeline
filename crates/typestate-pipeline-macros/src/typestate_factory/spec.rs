use syn::{Ident, Path, Type};

/// `#[factory(pipeline(carrier = …))]`.
pub struct PipelineSpec {
    pub carrier: Path,
}

/// `#[factory(finalize_async(via = …, into = …, error = …?))]`.
///
/// Generates an additional `async fn finalize_async()` that calls
/// `via(raw).await` (or `via(raw).await?` when `error` is supplied). The
/// sync `finalize()` is also retained.
pub struct FinalizeAsyncSpec {
    pub via: Ident,
    pub into: Type,
    pub error: Option<Type>,
}
