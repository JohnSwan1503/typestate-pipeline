#[path = "factory/minimal.rs"]
mod minimal;

#[path = "factory/required_and_optional.rs"]
mod required_and_optional;

#[path = "factory/default.rs"]
mod default;

#[path = "factory/setter_transformer.rs"]
mod setter_transformer;

#[path = "factory/setter_fallible.rs"]
mod setter_fallible;

#[path = "factory/setter_input_type.rs"]
mod setter_input_type;

#[path = "factory/setter_async.rs"]
mod setter_async;

#[path = "factory/removable.rs"]
mod removable;

#[path = "factory/overridable.rs"]
mod overridable;

#[path = "factory/internal.rs"]
mod internal;

#[path = "factory/rename.rs"]
mod rename;

#[path = "factory/finalize_async.rs"]
mod finalize_async;

#[path = "factory/ready_trait.rs"]
mod ready_trait;

#[path = "factory/empty_alias.rs"]
mod empty_alias;

// Gated to match the example's `required-features = ["no_unsafe"]`.
#[cfg(feature = "no_unsafe")]
#[path = "factory/no_unsafe.rs"]
mod no_unsafe;

#[path = "factory/pipeline_arm.rs"]
mod pipeline_arm;
