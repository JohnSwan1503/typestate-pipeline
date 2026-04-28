//! Runtime primitives for the typestate-pipeline ecosystem.
//!
//! - [`Pipeline`] is the dual-mode pipeline carrier; concrete pipelines are
//!   typically introduced as type aliases over it.
//! - [`Mode`], [`Resolved`], [`InFlight`], and [`BoxFuture`] form the mode
//!   axis that selects [`Pipeline`]'s storage shape.
//! - [`Pipelined`] is the marker trait that user-side carrier newtypes
//!   implement so the proc-macros can read carrier metadata via trait
//!   projection instead of AST manipulation.
//! - [`Yes`], [`No`], [`Satisfiable`], [`Satisfied`], and [`Storage`] are
//!   the per-field flag primitives the `TypestateFactory` derive uses for
//!   the named-field accumulator pattern.

pub mod flag;
pub mod mode;
pub mod pipeline;
pub mod pipelined;

pub use flag::{No, Satisfiable, Satisfied, Storage, Yes};
pub use mode::{BoxFuture, InFlight, Mode, Resolved};
pub use pipeline::Pipeline;
pub use pipelined::Pipelined;
