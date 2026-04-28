//! Mode axis: selects the storage shape for a [`Pipeline`](crate::Pipeline).
//!
//! - [`Resolved`] holds the state `S` directly.
//! - [`InFlight`] holds a pending future resolving to `Result<S, E>` and
//!   implements [`IntoFuture`] for it.
//!
//! [`IntoFuture`]: std::future::IntoFuture

use std::{future::Future, pin::Pin};

mod sealed {
    pub trait Mode {}
}

/// `Send`-able boxed future with lifetime `'a` — the storage type for
/// [`InFlight`]-mode pipelines.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Storage-shape selector for [`Pipeline`](crate::Pipeline).
///
/// Sealed; implemented only by [`Resolved`] and [`InFlight`].
pub trait Mode<'a, S, E>: sealed::Mode {
    /// The field type held by the pipeline under this mode.
    type Storage;
}

/// Mode marker: the pipeline holds resolved state data directly.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Resolved;

impl sealed::Mode for Resolved {}
impl<'a, S: 'a, E: 'a> Mode<'a, S, E> for Resolved {
    type Storage = S;
}

/// Mode marker: the pipeline holds a future resolving to the state data.
/// The pipeline implements [`IntoFuture`](std::future::IntoFuture) in this
/// mode.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InFlight;

impl sealed::Mode for InFlight {}
impl<'a, S: Send + 'a, E: Send + 'a> Mode<'a, S, E> for InFlight {
    type Storage = BoxFuture<'a, Result<S, E>>;
}
