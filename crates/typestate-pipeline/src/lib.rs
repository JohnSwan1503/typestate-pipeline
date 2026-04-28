//! Typestate-pipeline helpers — facade re-exporting the runtime carrier
//! and the proc-macros, plus declarative shorthand for declaring carrier
//! newtypes.
//!
//! - [`Pipeline`] — dual-mode carrier (Resolved / InFlight).
//! - [`Pipelined`] — marker trait the proc-macros use to introspect a
//!   carrier; emit the trait impl with [`pipelined!`] / [`impl_pipelined!`].
//! - [`transitions`] — attribute that generates Resolved + InFlight method
//!   pairs from a single source body.
//! - [`TypestateFactory`] — derive for the named-field accumulator pattern.
//!
//! Both proc-macros emit fully-qualified paths through this crate's
//! [`__private`] module as `::typestate_pipeline::__private::*`. The
//! `extern crate self as typestate_pipeline;` declaration below makes
//! that absolute path resolve from in-package uses (lib src, integration
//! tests, examples) as well as from downstream consumers. Renamed deps
//! (`helpers = { package = "typestate-pipeline" }`) are detected via
//! `proc_macro_crate` and routed through `::helpers::*`.

extern crate self as typestate_pipeline;

pub use typestate_pipeline_core::{
    BoxFuture, InFlight, Mode, No, Pipeline, Pipelined, Resolved, Satisfiable, Satisfied, Storage,
    Yes,
};
pub use typestate_pipeline_macros::{TypestateFactory, transitions};

/// Implementation detail. Items referenced by macro expansions.
///
/// Not part of the public API.
#[doc(hidden)]
pub mod __private {
    pub use core::marker::PhantomData;
    pub use core::mem::{ManuallyDrop, MaybeUninit};
    pub use core::pin::Pin;
    pub use core::ptr;
    pub use std::boxed::Box;

    pub use typestate_pipeline_core::{
        BoxFuture, InFlight, Mode, No, Pipeline, Pipelined, Resolved, Satisfiable, Satisfied,
        Storage, Yes,
    };
}

#[doc(hidden)]
#[cfg(feature = "dataset-authoring-example")]
pub mod dataset_authoring;

/// Declare a typestate carrier in one line: emits the newtype struct, its
/// `where M: Mode<…>` clause, the [`Pipelined`] impl, and the
/// [`IntoFuture`](core::future::IntoFuture) forwarding for `InFlight` mode.
///
/// ```ignore
/// typestate_pipeline::pipelined!(pub Author, ctx = Client, error = AuthoringError);
/// // optional: tag = MyTag (default `()`)
/// ```
///
/// Expands to:
///
/// ```ignore
/// pub struct Author<'a, S, M = Resolved>(Pipeline<'a, Client, (), S, AuthoringError, M>)
/// where M: Mode<'a, S, AuthoringError>;
/// // + Pipelined<'a> impl + IntoFuture forwarding
/// ```
///
/// Use [`impl_pipelined!`] when you need to hand-write the struct (custom
/// derives, extra generics like `<'a, K: Kind, S, M>`, etc.).
#[macro_export]
macro_rules! pipelined {
    ($vis:vis $name:ident, ctx = $ctx:ty, error = $err:ty $(,)?) => {
        $crate::pipelined!($vis $name, ctx = $ctx, error = $err, tag = ());
    };

    ($vis:vis $name:ident, ctx = $ctx:ty, error = $err:ty, tag = $tag:ty $(,)?) => {
        $vis struct $name<'a, S, M = $crate::Resolved>(
            $crate::Pipeline<'a, $ctx, $tag, S, $err, M>,
        )
        where
            M: $crate::Mode<'a, S, $err>;

        $crate::impl_pipelined!($name, ctx = $ctx, error = $err, tag = $tag);
    };
}

/// Implement [`Pipelined`] and [`IntoFuture`](core::future::IntoFuture) for
/// an existing carrier newtype, plus the chainable `inspect` combinator on
/// both `Resolved` and `InFlight` modes.
///
/// The carrier must be a tuple-struct newtype around `Pipeline` with the
/// generic shape `<'a, S, M = Resolved>`:
///
/// ```ignore
/// struct Author<'a, S, M = Resolved>(Pipeline<'a, Client, (), S, AuthoringError, M>)
/// where M: Mode<'a, S, AuthoringError>;
///
/// typestate_pipeline::impl_pipelined!(Author, ctx = Client, error = AuthoringError);
/// ```
///
/// For the common case where you do not need to customize the struct, use
/// [`pipelined!`] which emits both in one line.
#[macro_export]
macro_rules! impl_pipelined {
    ($carrier:ident, ctx = $ctx:ty, error = $err:ty $(,)?) => {
        $crate::impl_pipelined!($carrier, ctx = $ctx, error = $err, tag = ());
    };

    ($carrier:ident, ctx = $ctx:ty, error = $err:ty, tag = $tag:ty $(,)?) => {
        impl<'a, S: 'a, M> $crate::Pipelined<'a> for $carrier<'a, S, M>
        where
            M: $crate::Mode<'a, S, $err>,
        {
            type Ctx = $ctx;
            type Error = $err;
            type Tag = $tag;
            type State = S;
            type Mode = M;
            type Resolved<NS: 'a> = $carrier<'a, NS, $crate::Resolved>;
            type InFlight<NS: ::core::marker::Send + 'a> = $carrier<'a, NS, $crate::InFlight>;
        }

        impl<'a, S> ::core::future::IntoFuture for $carrier<'a, S, $crate::InFlight>
        where
            S: ::core::marker::Send + 'a,
            $err: ::core::marker::Send + 'a,
            $ctx: ::core::marker::Sync + 'a,
        {
            type Output = ::core::result::Result<$carrier<'a, S, $crate::Resolved>, $err>;
            type IntoFuture = $crate::BoxFuture<'a, Self::Output>;
            fn into_future(self) -> Self::IntoFuture {
                let pending = self.0;
                $crate::__private::Box::pin(async move {
                    let resolved = pending.await?;
                    ::core::result::Result::Ok($carrier(resolved))
                })
            }
        }

        impl<'a, S: 'a> $carrier<'a, S, $crate::Resolved> {
            /// Pause the chain to inspect the resolved carrier without
            /// changing it. The closure receives `&Self`; the carrier is
            /// returned unchanged so the chain continues.
            #[inline]
            pub fn inspect<F>(self, inspect_op: F) -> Self
            where
                F: ::core::ops::FnOnce(&Self),
            {
                inspect_op(&self);
                self
            }
        }

        impl<'a, S> $carrier<'a, S, $crate::InFlight>
        where
            S: ::core::marker::Send + 'a,
            $err: ::core::marker::Send + 'a,
            $ctx: ::core::marker::Sync + 'a,
        {
            /// Pause the in-flight chain to inspect the eventual resolved
            /// carrier without changing it.
            ///
            /// The closure runs *after* the chain's pending future
            /// resolves, against a temporary [`Resolved`](crate::Resolved)
            /// carrier reference (so getters on the carrier work). The
            /// chain re-enters `InFlight` so subsequent transitions
            /// continue folding into the same terminal `.await?`.
            #[inline]
            pub fn inspect<F>(self, inspect_op: F) -> Self
            where
                F: ::core::ops::FnOnce(&$carrier<'a, S, $crate::Resolved>)
                    + ::core::marker::Send
                    + 'a,
            {
                let pending = self.0;
                let ctx = pending.ctx();
                $carrier($crate::__private::Pipeline::in_flight(
                    ctx,
                    $crate::__private::Box::pin(async move {
                        let resolved = pending.await?;
                        let temp = $carrier(resolved);
                        inspect_op(&temp);
                        ::core::result::Result::Ok(temp.0.into_state())
                    }),
                ))
            }
        }
    };
}
