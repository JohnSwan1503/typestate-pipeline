#[path = "bookkeeping.rs"]
pub mod bookkeeping;

#[path = "panicky_drop.rs"]
pub mod panicky_drop;

pub use bookkeeping::{COUNTED_ALIVE, Counted, LOCK, PANICKY_FUSE, alive, setup};
pub use panicky_drop::PanickyDrop;
