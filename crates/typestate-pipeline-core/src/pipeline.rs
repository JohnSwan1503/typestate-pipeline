//! Generic dual-mode pipeline carrier.
//!
//! Concrete pipelines are typically introduced as type aliases:
//!
//! ```ignore
//! type DatasetAuthor<'a, K, S, M = Resolved> =
//!     Pipeline<'a, Client, K, S, AuthoringError, M>;
//! ```
//!
//! Transitions are written in `impl` blocks decorated with `#[transitions]`
//! from `typestate-pipeline-macros`.

use std::{future::IntoFuture, marker::PhantomData};

use crate::mode::{BoxFuture, InFlight, Mode, Resolved};

/// Dual-mode pipeline carrier.
///
/// - `Ctx`: borrowed context (e.g. an admin client).
/// - `Tag`: phantom marker (typically the dataset/object kind).
/// - `S`: current state.
/// - `E`: error type used by `InFlight` futures and fallible transitions.
/// - `M`: [`Mode`] (default [`Resolved`]).
///
/// Fields are private — use [`Pipeline::resolved`] / [`Pipeline::in_flight`]
/// to construct, [`Pipeline::ctx`] / [`Pipeline::into_parts`] (and the
/// Resolved-mode [`state`](Pipeline::state) / [`into_state`](Pipeline::into_state)
/// accessors) to read. Sealing the fields means a downstream user's
/// carrier newtype can't accidentally bypass the typestate machinery by
/// hand-substituting `inner` or forging a `_tag`.
pub struct Pipeline<'a, Ctx, Tag, S, E, M = Resolved>
where
    Ctx: ?Sized,
    M: Mode<'a, S, E>,
{
    /// Borrowed context. The `transitions` macro reads this name when
    /// expanding `ctx = …` references in user bodies.
    ctx: &'a Ctx,
    /// State storage — `S` under [`Resolved`], a `Result<S, E>` future
    /// under [`InFlight`].
    inner: M::Storage,
    // `fn() -> _` makes Tag and E covariant and prevents auto-trait
    // inheritance from those generics — the carrier behaves like a phantom
    // marker, not a value.
    _tag: PhantomData<fn() -> Tag>,
    _err: PhantomData<fn() -> E>,
}

impl<'a, Ctx, Tag, S, E, M> Pipeline<'a, Ctx, Tag, S, E, M>
where
    Ctx: ?Sized,
    M: Mode<'a, S, E>,
{
    /// Borrow the pipeline's context.
    ///
    /// Defined for both [`Resolved`] and [`InFlight`] modes — generated
    /// code on user carriers calls this through the carrier's tuple
    /// field instead of reaching into [`Pipeline`]'s now-private fields.
    pub fn ctx(&self) -> &'a Ctx {
        self.ctx
    }

    /// Consume the pipeline, returning the borrowed context and the
    /// mode-specific inner storage (the state under [`Resolved`], the
    /// pending future under [`InFlight`]).
    ///
    /// This is the entry point the proc-macros use to destructure the
    /// pipeline without touching its private fields.
    pub fn into_parts(self) -> (&'a Ctx, M::Storage) {
        (self.ctx, self.inner)
    }
}

impl<'a, Ctx, Tag, S, E> Pipeline<'a, Ctx, Tag, S, E, Resolved>
where
    Ctx: ?Sized + 'a,
    S: 'a,
    E: 'a,
{
    /// Construct a Resolved pipeline directly around a state value.
    pub fn resolved(ctx: &'a Ctx, state: S) -> Self {
        Pipeline {
            ctx,
            inner: state,
            _tag: PhantomData,
            _err: PhantomData,
        }
    }

    /// Borrow the state.
    pub fn state(&self) -> &S {
        &self.inner
    }

    /// Mutably borrow the state.
    pub fn state_mut(&mut self) -> &mut S {
        &mut self.inner
    }

    /// Consume the pipeline, returning its state.
    pub fn into_state(self) -> S {
        self.inner
    }
}

impl<'a, Ctx, Tag, S, E> Pipeline<'a, Ctx, Tag, S, E, InFlight>
where
    Ctx: ?Sized + Sync + 'a,
    S: Send + 'a,
    E: Send + 'a,
{
    /// Construct an InFlight pipeline around a pending future.
    pub fn in_flight(ctx: &'a Ctx, fut: BoxFuture<'a, Result<S, E>>) -> Self {
        Pipeline {
            ctx,
            inner: fut,
            _tag: PhantomData,
            _err: PhantomData,
        }
    }
}

impl<'a, Ctx, Tag, S, E> IntoFuture for Pipeline<'a, Ctx, Tag, S, E, InFlight>
where
    Ctx: ?Sized + Sync + 'a,
    Tag: 'a,
    S: Send + 'a,
    E: Send + 'a,
{
    type Output = Result<Pipeline<'a, Ctx, Tag, S, E, Resolved>, E>;
    type IntoFuture = BoxFuture<'a, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        let ctx = self.ctx;
        let pending = self.inner;
        Box::pin(async move {
            let state = pending.await?;
            Ok(Pipeline::resolved(ctx, state))
        })
    }
}

impl<'a, Ctx, Tag, S, E> Pipeline<'a, Ctx, Tag, S, E, InFlight>
where
    Ctx: ?Sized + Sync + 'a,
    Tag: 'a,
    S: Send + 'a,
    E: Send + 'a,
{
    /// Chain a sync infallible transition through the pending future.
    ///
    /// Used by `#[transitions]` to expand the InFlight arm of a sync
    /// infallible transition: await the prior pending state, apply `f`,
    /// re-wrap in a new pending future.
    pub fn map_inner_sync<S2, F>(self, f: F) -> Pipeline<'a, Ctx, Tag, S2, E, InFlight>
    where
        S2: Send + 'a,
        F: FnOnce(S) -> S2 + Send + 'a,
    {
        let ctx = self.ctx;
        let pending = self.inner;
        Pipeline::in_flight(
            ctx,
            Box::pin(async move {
                let state = pending.await?;
                Ok(f(state))
            }),
        )
    }

    /// Chain a sync fallible transition through the pending future. The
    /// `Result<S2, E>` returned by `f` folds into the pending future's
    /// own `Result`.
    pub fn map_inner_sync_fallible<S2, F>(self, f: F) -> Pipeline<'a, Ctx, Tag, S2, E, InFlight>
    where
        S2: Send + 'a,
        F: FnOnce(S) -> Result<S2, E> + Send + 'a,
    {
        let ctx = self.ctx;
        let pending = self.inner;
        Pipeline::in_flight(
            ctx,
            Box::pin(async move {
                let state = pending.await?;
                f(state)
            }),
        )
    }
}

impl<'a, Ctx, Tag, S, E> std::fmt::Debug for Pipeline<'a, Ctx, Tag, S, E, Resolved>
where
    Ctx: ?Sized,
    S: std::fmt::Debug + 'a,
    E: 'a,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pipeline")
            .field("state", &self.inner)
            .finish_non_exhaustive()
    }
}

impl<'a, Ctx, Tag, S, E> std::fmt::Debug for Pipeline<'a, Ctx, Tag, S, E, InFlight>
where
    Ctx: ?Sized,
    S: Send + 'a,
    E: Send + 'a,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pipeline<InFlight>").finish_non_exhaustive()
    }
}
