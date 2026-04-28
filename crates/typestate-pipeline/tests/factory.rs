//! Feature coverage for `#[derive(TypestateFactory)]`: setters, getters,
//! defaults, the auto-generated `Ready` trait, async finalize, `input = …`
//! routing, and `internal` fields.

#[path = "factory/async.rs"]
mod r#async;
#[path = "factory/core.rs"]
mod core;
#[path = "factory/input_type.rs"]
mod input_type;
#[path = "factory/internal_field.rs"]
mod internal_field;
#[path = "factory/ready_trait.rs"]
mod ready_trait;
