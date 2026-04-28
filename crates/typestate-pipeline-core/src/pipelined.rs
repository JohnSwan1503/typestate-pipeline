//! [`Pipelined<'a>`] — marker trait that user-side typestate carriers
//! implement so the proc-macros can introspect them.
//!
//! A *carrier* is a tuple-struct newtype around [`Pipeline`](crate::Pipeline)
//! the user declares to escape orphan rules:
//!
//! ```ignore
//! struct Author<'a, S, M = Resolved>(Pipeline<'a, Client, (), S, AuthoringError, M>)
//! where M: Mode<'a, S, AuthoringError>;
//! ```
//!
//! Implementing [`Pipelined<'a>`] exposes the carrier's [`Ctx`], [`Error`],
//! [`Tag`], [`State`], and [`Mode`] plus the GAT projections
//! [`Resolved`](Pipelined::Resolved) and [`InFlight`](Pipelined::InFlight)
//! that map a destination state to the correctly-moded carrier type. The
//! proc-macros use those projections instead of mutating the user's
//! self-type AST.
//!
//! Hand-implementing the trait is mechanical; the `pipelined!` and
//! `impl_pipelined!` macros in the `typestate-pipeline` facade emit it (with
//! [`IntoFuture`] forwarding) for the conventional carrier shape.
//!
//! [`Ctx`]: Pipelined::Ctx
//! [`Error`]: Pipelined::Error
//! [`Tag`]: Pipelined::Tag
//! [`State`]: Pipelined::State
//! [`Mode`]: Pipelined::Mode
//! [`IntoFuture`]: core::future::IntoFuture

use crate::mode::{InFlight, Mode, Resolved};

/// Marker trait for typestate carrier newtypes.
///
/// `Resolved<NS>` and `InFlight<NS>` project a destination *state* to the
/// fully-instantiated carrier type with the appropriate mode. A macro can
/// emit `<Self as Pipelined<'a>>::Resolved<NextState>` instead of reaching
/// into the user's self-type AST and replacing the last generic argument —
/// the projection is type-system-enforced, so a carrier with unusual
/// generic ordering or extra type parameters works as long as the impl is
/// correct.
pub trait Pipelined<'a>: Sized {
    /// Borrowed context type — usually a client/handle the pipeline reads
    /// during transitions.
    type Ctx: ?Sized + 'a;

    /// Error type used by fallible transitions and InFlight futures.
    type Error: 'a;

    /// Phantom kind/tag — usually `()`, sometimes a type-level marker.
    type Tag: 'a;

    /// The carrier's current state.
    type State: 'a;

    /// The carrier's current mode (`Resolved` or `InFlight`).
    type Mode: Mode<'a, Self::State, Self::Error>;

    /// Carrier type for the *same* `Ctx`/`Error`/`Tag`, with state `NS`
    /// and mode `Resolved`.
    type Resolved<NS: 'a>: Pipelined<
        'a,
        Ctx = Self::Ctx,
        Error = Self::Error,
        Tag = Self::Tag,
        State = NS,
        Mode = Resolved,
    >;

    /// Carrier type for the *same* `Ctx`/`Error`/`Tag`, with state `NS`
    /// and mode `InFlight`. Only valid when `Self::Ctx: Sync` and
    /// `Self::Error: Send` (the `InFlight` mode's bounds).
    type InFlight<NS: Send + 'a>: Pipelined<
        'a,
        Ctx = Self::Ctx,
        Error = Self::Error,
        Tag = Self::Tag,
        State = NS,
        Mode = InFlight,
    >
    where
        Self::Ctx: Sync,
        Self::Error: Send;
}
