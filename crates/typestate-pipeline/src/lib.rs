#![cfg_attr(docsrs, feature(doc_cfg, doc_notable_trait))]
#![doc = include_str!("../../../README.md")]
//!
//! ---
//!
//! ### Implementation note
//!
//! Both proc-macros emit fully-qualified paths through this crate's
//! [`__private`] module as `::typestate_pipeline::__private::*`. The
//! `extern crate self as typestate_pipeline;` declaration below makes
//! that absolute path resolve from in-package uses (lib src, integration
//! tests, examples) as well as from downstream consumers. Renamed deps
//! (`helpers = { package = "typestate-pipeline" }`) are detected via
//! `proc_macro_crate` and routed through `::helpers::*`.

extern crate self as typestate_pipeline;

#[doc(inline)]
pub use typestate_pipeline_core::{
    BoxFuture, InFlight, Mode, No, Pipeline, Pipelined, Resolved, Satisfiable, Satisfied, Storage,
    Yes,
};
#[doc(inline)]
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
#[cfg_attr(docsrs, doc(cfg(feature = "dataset-authoring-example")))]
pub mod dataset_authoring;

/// One continuous narrative covering every macro the crate
/// offers — the factory first, then `#[transitions]`, then the
/// carrier macros (`pipelined!` / `impl_pipelined!`), then the
/// combinations. Each section pairs runnable source (the same
/// files the [test suite] compiles) with a sketch of the
/// generated surface, so you can read what the macros emit
/// without running `cargo expand`.
///
/// Only present in rustdoc builds — `guide` is not an
/// importable path in downstream crates.
///
/// [test suite]: https://github.com/JohnSwan1503/typestate-pipeline/tree/main/crates/typestate-pipeline/tests/expansions
#[cfg(any(doc, docsrs))]
pub mod guide {
    #![doc = include_str!("../examples/expansions/lead.md")]
    //!
    #![doc = include_str!("../examples/expansions/factory/intro.md")]
    //!
    #![doc = include_str!("../examples/expansions/factory/minimal.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/factory/minimal.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/factory/minimal.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/factory/required_and_optional.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/factory/required_and_optional.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/factory/required_and_optional.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/factory/default.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/factory/default.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/factory/default.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/factory/removable.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/factory/removable.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/factory/removable.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/factory/overridable.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/factory/overridable.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/factory/overridable.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/factory/rename.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/factory/rename.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/factory/rename.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/factory/setter_transformer.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/factory/setter_transformer.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/factory/setter_transformer.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/factory/setter_input_type.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/factory/setter_input_type.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/factory/setter_input_type.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/factory/setter_fallible.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/factory/setter_fallible.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/factory/setter_fallible.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/factory/setter_async.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/factory/setter_async.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/factory/setter_async.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/factory/internal.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/factory/internal.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/factory/internal.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/factory/finalize_async.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/factory/finalize_async.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/factory/finalize_async.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/factory/ready_trait.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/factory/ready_trait.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/factory/ready_trait.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/factory/empty_alias.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/factory/empty_alias.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/factory/empty_alias.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/factory/pipeline_arm.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/factory/pipeline_arm.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/factory/pipeline_arm.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/factory/no_unsafe.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/factory/no_unsafe.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/factory/no_unsafe.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/transitions/intro.md")]
    //!
    #![doc = include_str!("../examples/expansions/transitions/sync_infallible.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/transitions/sync_infallible.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/transitions/sync_infallible.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/transitions/sync_fallible.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/transitions/sync_fallible.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/transitions/sync_fallible.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/transitions/async_deferred.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/transitions/async_deferred.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/transitions/async_deferred.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/transitions/async_breakpoint.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/transitions/async_breakpoint.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/transitions/async_breakpoint.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/pipelined/intro.md")]
    //!
    #![doc = include_str!("../examples/expansions/pipelined/minimal.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/pipelined/minimal.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/pipelined/minimal.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/pipelined/inspect_combinator.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/pipelined/inspect_combinator.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/pipelined/inspect_combinator.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/pipelined/with_tag.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/pipelined/with_tag.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/pipelined/with_tag.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/impl_pipelined/intro.md")]
    //!
    #![doc = include_str!("../examples/expansions/impl_pipelined/minimal.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/impl_pipelined/minimal.rs")]
    //! ```
    //!
    //! **Generated (sketch):**
    //!
    #![doc = include_str!("../examples/expansions/impl_pipelined/minimal.expansion.md")]
    //!
    #![doc = include_str!("../examples/expansions/combo/intro.md")]
    //!
    #![doc = include_str!("../examples/expansions/combo/factory_in_pipeline.description.md")]
    //!
    //! ```rust
    #![doc = include_str!("../examples/expansions/combo/factory_in_pipeline.rs")]
    //! ```
}

/// The integration test catalog — every test file rendered as a
/// browsable page so you can see exactly what behavior is locked in.
///
/// Tests are grouped by domain. UI tests pair the trybuild source with
/// the pinned `.stderr` snapshot so you can see the exact diagnostic
/// users get for each kind of misuse.
///
/// Only present in rustdoc builds — `tests` is not an importable path
/// in downstream crates.
#[cfg(any(doc, docsrs))]
pub mod tests {
    /// `#[derive(TypestateFactory)]` feature coverage.
    pub mod factory {
        /// Construction & getters, Drop semantics, removable/overridable,
        /// defaults & conditional finalize, custom names, and sync
        /// transformers (infallible + fallible).
        ///
        #[doc = include_str!("../tests/factory/core/docs/tests.md")]
        ///
        /// ## Test Cases
        ///
        #[doc = include_str!("../tests/factory/core/docs/build_in_order.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/build_in_order.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/build_in_arbitrary_order.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/build_in_arbitrary_order.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/default_helper_fills_in_optional.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/default_helper_fills_in_optional.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/getter_borrows_set_field.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/getter_borrows_set_field.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/empty_bag_dropped_does_not_touch_uninit_fields.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/empty_bag_dropped_does_not_touch_uninit_fields.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/partial_bag_dropped_drops_only_set_fields.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/partial_bag_dropped_drops_only_set_fields.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/fully_populated_bag_dropped_drops_all.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/fully_populated_bag_dropped_drops_all.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/finalize_does_not_double_drop.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/finalize_does_not_double_drop.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/drop_field_drops_value_once.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/drop_field_drops_value_once.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/drop_field_then_reset_doesnt_double_drop.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/drop_field_then_reset_doesnt_double_drop.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/override_drops_old_value.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/override_drops_old_value.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/finalize_uses_defaults_when_optional_no.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/finalize_uses_defaults_when_optional_no.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/finalize_keeps_explicit_values_when_optional_yes.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/finalize_keeps_explicit_values_when_optional_yes.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/finalize_mixes_set_and_default.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/finalize_mixes_set_and_default.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/custom_bag_name.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/custom_bag_name.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/custom_setter_name.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/custom_setter_name.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/custom_transformer_fn.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/custom_transformer_fn.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/fallible_transformer_success.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/fallible_transformer_success.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/core/docs/fallible_transformer_failure.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/fallible_transformer_failure.rs")]
        /// ```
        ///
        /// ## Shared types and setup
        ///
        #[doc = include_str!("../tests/factory/core/docs/shared.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/core/tests/shared.rs")]
        /// ```
        pub mod core {}

        /// `async_fn` setters (success and failure), `finalize_async`,
        /// pipeline-arm async setters threading through `InFlight`, and
        /// `#[transitions]` bodies that call `.finalize()` mid-chain.
        ///
        #[doc = include_str!("../tests/factory/async/docs/tests.md")]
        ///
        /// ## Test Cases
        ///
        #[doc = include_str!("../tests/factory/async/docs/standalone_async_setter_non_fallible.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/async/tests/standalone_async_setter_non_fallible.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/async/docs/standalone_async_setter_fallible_failure.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/async/tests/standalone_async_setter_fallible_failure.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/async/docs/standalone_async_finalize.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/async/tests/standalone_async_finalize.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/async/docs/pipeline_async_setter_chains_through_inflight.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/async/tests/pipeline_async_setter_chains_through_inflight.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/async/docs/pipeline_async_fallible_setter_propagates_error.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/async/tests/pipeline_async_fallible_setter_propagates_error.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/async/docs/transitions_body_calls_finalize.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/async/tests/transitions_body_calls_finalize.rs")]
        /// ```
        ///
        /// ## Shared types and setup
        ///
        #[doc = include_str!("../tests/factory/async/docs/error.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/async/tests/error.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/async/docs/bags.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/async/tests/bags.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/async/docs/carrier.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/async/tests/carrier.rs")]
        /// ```
        pub mod async_setters {}

        /// `input = …` setter input types and how they bridge to the
        /// storage type via the transformer.
        ///
        #[doc = include_str!("../tests/factory/input_type/docs/tests.md")]
        ///
        /// ## Test Cases
        ///
        #[doc = include_str!("../tests/factory/input_type/docs/setter_takes_input_type_not_field_type.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/input_type/tests/setter_takes_input_type_not_field_type.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/input_type/docs/default_helper_bypasses_transformer.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/input_type/tests/default_helper_bypasses_transformer.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/input_type/docs/unset_default_field_uses_default_at_finalize.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/input_type/tests/unset_default_field_uses_default_at_finalize.rs")]
        /// ```
        ///
        /// ## Shared types and setup
        ///
        #[doc = include_str!("../tests/factory/input_type/docs/shared.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/input_type/tests/shared.rs")]
        /// ```
        pub mod input_type {}

        /// `internal` field positional-on-`new`, dropped from the
        /// flag-generic list, with unconditional getters on both bag
        /// and carrier arms.
        ///
        #[doc = include_str!("../tests/factory/internal_field/docs/tests.md")]
        ///
        /// ## Test Cases
        ///
        #[doc = include_str!("../tests/factory/internal_field/docs/constructor_takes_internal_field_as_argument.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/internal_field/tests/constructor_takes_internal_field_as_argument.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/internal_field/docs/internal_getter_is_unconditional.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/internal_field/tests/internal_getter_is_unconditional.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/internal_field/docs/internal_field_dropped_from_flag_generic_list.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/internal_field/tests/internal_field_dropped_from_flag_generic_list.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/internal_field/docs/pipeline_arm_works_for_non_internal_fields.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/internal_field/tests/pipeline_arm_works_for_non_internal_fields.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/internal_field/docs/finalize_passes_internal_field_through.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/internal_field/tests/finalize_passes_internal_field_through.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/internal_field/docs/carrier_internal_getter_is_unconditional.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/internal_field/tests/carrier_internal_getter_is_unconditional.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/internal_field/docs/carrier_non_internal_getter_gates_on_yes_flag.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/internal_field/tests/carrier_non_internal_getter_gates_on_yes_flag.rs")]
        /// ```
        ///
        /// ## Shared types and setup
        ///
        #[doc = include_str!("../tests/factory/internal_field/docs/error.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/internal_field/tests/error.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/internal_field/docs/carrier.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/internal_field/tests/carrier.rs")]
        /// ```
        pub mod internal_field {}

        /// `<Bag>Ready` companion trait — auto-impl matches `finalize`'s
        /// bounds; generic `B: <Bag>Ready` dispatch matches inherent
        /// `finalize()`.
        ///
        #[doc = include_str!("../tests/factory/ready_trait/docs/tests.md")]
        ///
        /// ## Test Cases
        ///
        #[doc = include_str!("../tests/factory/ready_trait/docs/ready_trait_is_implemented_when_required_flags_yes.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/ready_trait/tests/ready_trait_is_implemented_when_required_flags_yes.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/ready_trait/docs/ready_trait_works_when_optional_set_too.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/ready_trait/tests/ready_trait_works_when_optional_set_too.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/factory/ready_trait/docs/dispatch_via_trait_matches_inherent_finalize.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/ready_trait/tests/dispatch_via_trait_matches_inherent_finalize.rs")]
        /// ```
        ///
        /// ## Shared types and setup
        ///
        #[doc = include_str!("../tests/factory/ready_trait/docs/shared.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/factory/ready_trait/tests/shared.rs")]
        /// ```
        pub mod ready_trait {}
    }

    /// `#[transitions]` body shapes and `Pipelined` resolution.
    pub mod transitions {
        /// Full Resolved → InFlight → Resolved chain folding,
        /// breakpoint `breakpoint` forcing explicit awaits, sync
        /// fallible arms, `IntoFuture` driving InFlight back to
        /// Resolved.
        ///
        #[doc = include_str!("../tests/transitions/core/docs/tests.md")]
        ///
        /// ## Test Cases
        ///
        #[doc = include_str!("../tests/transitions/core/docs/full_chain_with_resolved_breakpoint_in_middle.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/transitions/core/tests/full_chain_with_resolved_breakpoint_in_middle.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/transitions/core/docs/breakpoint_forces_explicit_await.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/transitions/core/tests/breakpoint_forces_explicit_await.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/transitions/core/docs/sync_fallible_resolved_returns_result_directly.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/transitions/core/tests/sync_fallible_resolved_returns_result_directly.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/transitions/core/docs/sync_fallible_propagates_through_inflight_chain.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/transitions/core/tests/sync_fallible_propagates_through_inflight_chain.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/transitions/core/docs/intofuture_resolves_inflight_back_to_resolved.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/transitions/core/tests/intofuture_resolves_inflight_back_to_resolved.rs")]
        /// ```
        ///
        /// ## Shared types and setup
        ///
        #[doc = include_str!("../tests/transitions/core/docs/error.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/transitions/core/tests/error.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/transitions/core/docs/phases.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/transitions/core/tests/phases.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/transitions/core/docs/carrier.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/transitions/core/tests/carrier.rs")]
        /// ```
        pub mod core {}

        /// Chains with and without an `error =` arg; factory carriers
        /// without an error type still get the pipeline arm;
        /// `Pipelined` supplies the `IntoFuture`.
        ///
        #[doc = include_str!("../tests/transitions/via_pipelined/docs/tests.md")]
        ///
        /// ## Test Cases
        ///
        #[doc = include_str!("../tests/transitions/via_pipelined/docs/transitions_chain_without_error_arg.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/transitions/via_pipelined/tests/transitions_chain_without_error_arg.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/transitions/via_pipelined/docs/transitions_chain_propagates_error.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/transitions/via_pipelined/tests/transitions_chain_propagates_error.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/transitions/via_pipelined/docs/factory_pipeline_arms_without_error_arg.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/transitions/via_pipelined/tests/factory_pipeline_arms_without_error_arg.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/transitions/via_pipelined/docs/intofuture_provided_by_pipelined.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/transitions/via_pipelined/tests/intofuture_provided_by_pipelined.rs")]
        /// ```
        ///
        /// ## Shared types and setup
        ///
        #[doc = include_str!("../tests/transitions/via_pipelined/docs/error.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/transitions/via_pipelined/tests/error.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/transitions/via_pipelined/docs/phases.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/transitions/via_pipelined/tests/phases.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/transitions/via_pipelined/docs/carrier.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/transitions/via_pipelined/tests/carrier.rs")]
        /// ```
        pub mod via_pipelined {}

        /// Attributes on the source `impl` block forwarded to both
        /// generated arms.
        ///
        #[doc = include_str!("../tests/transitions/attr_forwarding/docs/tests.md")]
        ///
        /// ## Test Cases
        ///
        #[doc = include_str!("../tests/transitions/attr_forwarding/docs/impl_attr_forwarded_to_both_arms.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/transitions/attr_forwarding/tests/impl_attr_forwarded_to_both_arms.rs")]
        /// ```
        pub mod attr_forwarding {}
    }

    /// Cross-feature scenarios — factory + pipeline + transitions
    /// composing together.
    pub mod integration {
        /// Pipeline-arm setters from `#[factory(pipeline(carrier = …))]`:
        /// the bag's setters / removers / overriders reach the user's
        /// carrier in both Resolved and InFlight modes.
        ///
        #[doc = include_str!("../tests/integration/factory_pipeline/docs/tests.md")]
        ///
        /// ## Test Cases
        ///
        #[doc = include_str!("../tests/integration/factory_pipeline/docs/pipeline_setters_chain_in_resolved_mode.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/factory_pipeline/tests/pipeline_setters_chain_in_resolved_mode.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/integration/factory_pipeline/docs/pipeline_setters_chain_through_inflight.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/factory_pipeline/tests/pipeline_setters_chain_through_inflight.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/integration/factory_pipeline/docs/pipeline_drop_field_transitions_yes_to_no.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/factory_pipeline/tests/pipeline_drop_field_transitions_yes_to_no.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/integration/factory_pipeline/docs/pipeline_override_replaces_value.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/factory_pipeline/tests/pipeline_override_replaces_value.rs")]
        /// ```
        ///
        /// ## Shared types and setup
        ///
        #[doc = include_str!("../tests/integration/factory_pipeline/docs/error.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/factory_pipeline/tests/error.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/integration/factory_pipeline/docs/domain.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/factory_pipeline/tests/domain.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/integration/factory_pipeline/docs/carrier.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/factory_pipeline/tests/carrier.rs")]
        /// ```
        pub mod factory_pipeline {}

        /// Standalone bag's `finalize()` feeding a `#[transitions]` body
        /// mid-chain; bag-level fallible finalize short-circuiting the
        /// surrounding pipeline chain.
        ///
        #[doc = include_str!("../tests/integration/factory_in_pipeline/docs/tests.md")]
        ///
        /// ## Test Cases
        ///
        #[doc = include_str!("../tests/integration/factory_in_pipeline/docs/full_chain_bag_into_pipeline.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/factory_in_pipeline/tests/full_chain_bag_into_pipeline.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/integration/factory_in_pipeline/docs/validation_failure_at_bag_finalize.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/factory_in_pipeline/tests/validation_failure_at_bag_finalize.rs")]
        /// ```
        ///
        /// ## Shared types and setup
        ///
        #[doc = include_str!("../tests/integration/factory_in_pipeline/docs/error.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/factory_in_pipeline/tests/error.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/integration/factory_in_pipeline/docs/domain.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/factory_in_pipeline/tests/domain.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/integration/factory_in_pipeline/docs/phases.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/factory_in_pipeline/tests/phases.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/integration/factory_in_pipeline/docs/carrier.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/factory_in_pipeline/tests/carrier.rs")]
        /// ```
        pub mod factory_in_pipeline {}

        /// The full multi-phase authoring pipeline (gated on
        /// `dataset-authoring-example`), patch-bump arithmetic, and typed
        /// error variants on kind mismatch.
        ///
        #[doc = include_str!("../tests/integration/dataset_authoring/docs/tests.md")]
        ///
        /// ## Test Cases
        ///
        #[doc = include_str!("../tests/integration/dataset_authoring/docs/new_evm_rpc_flow_terminates_at_deployed.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/dataset_authoring/tests/new_evm_rpc_flow_terminates_at_deployed.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/integration/dataset_authoring/docs/new_derived_flow_chains_through_single_await.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/dataset_authoring/tests/new_derived_flow_chains_through_single_await.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/integration/dataset_authoring/docs/bump_patch_increments_existing_version.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/dataset_authoring/tests/bump_patch_increments_existing_version.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/integration/dataset_authoring/docs/bump_patch_errors_when_no_prior_version.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/dataset_authoring/tests/bump_patch_errors_when_no_prior_version.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/integration/dataset_authoring/docs/edit_existing_kind_mismatch_surfaces_error.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/dataset_authoring/tests/edit_existing_kind_mismatch_surfaces_error.rs")]
        /// ```
        ///
        /// ## Shared types and setup
        ///
        #[doc = include_str!("../tests/integration/dataset_authoring/docs/shared.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/integration/dataset_authoring/tests/shared.rs")]
        /// ```
        pub mod dataset_authoring {}
    }

    /// Dual-mode `Pipeline` carrier and `inspect` combinator.
    pub mod pipeline {
        /// `Pipelined` associated types and GAT projections,
        /// `IntoFuture` driving InFlight to Resolved, carriers with
        /// extra generics still satisfying `Pipelined`.
        ///
        #[doc = include_str!("../tests/pipeline/impl_pipelined/docs/tests.md")]
        ///
        /// ## Test Cases
        ///
        #[doc = include_str!("../tests/pipeline/impl_pipelined/docs/pipelined_associated_types_resolve.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/pipeline/impl_pipelined/tests/pipelined_associated_types_resolve.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/pipeline/impl_pipelined/docs/gat_projections_are_correct.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/pipeline/impl_pipelined/tests/gat_projections_are_correct.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/pipeline/impl_pipelined/docs/intofuture_drives_inflight_back_to_resolved.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/pipeline/impl_pipelined/tests/intofuture_drives_inflight_back_to_resolved.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/pipeline/impl_pipelined/docs/tagged_pipelined_resolves.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/pipeline/impl_pipelined/tests/tagged_pipelined_resolves.rs")]
        /// ```
        ///
        /// ## Shared types and setup
        ///
        #[doc = include_str!("../tests/pipeline/impl_pipelined/docs/error.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/pipeline/impl_pipelined/tests/error.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/pipeline/impl_pipelined/docs/state_types.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/pipeline/impl_pipelined/tests/state_types.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/pipeline/impl_pipelined/docs/carriers.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/pipeline/impl_pipelined/tests/carriers.rs")]
        /// ```
        pub mod impl_pipelined {}

        /// `inspect(|c| …)` runs synchronously on Resolved, runs after
        /// the pending future resolves on InFlight, preserves the
        /// chain in both cases.
        ///
        #[doc = include_str!("../tests/pipeline/inspect/docs/tests.md")]
        ///
        /// ## Test Cases
        ///
        #[doc = include_str!("../tests/pipeline/inspect/docs/resolved_inspect_runs_sync_and_preserves_chain.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/pipeline/inspect/tests/resolved_inspect_runs_sync_and_preserves_chain.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/pipeline/inspect/docs/resolved_inspect_does_not_change_typestate.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/pipeline/inspect/tests/resolved_inspect_does_not_change_typestate.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/pipeline/inspect/docs/inflight_inspect_runs_after_pending_resolves.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/pipeline/inspect/tests/inflight_inspect_runs_after_pending_resolves.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/pipeline/inspect/docs/inflight_inspect_chains_through_subsequent_transitions.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/pipeline/inspect/tests/inflight_inspect_chains_through_subsequent_transitions.rs")]
        /// ```
        ///
        /// ## Shared types and setup
        ///
        #[doc = include_str!("../tests/pipeline/inspect/docs/error.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/pipeline/inspect/tests/error.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/pipeline/inspect/docs/phases.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/pipeline/inspect/tests/phases.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/pipeline/inspect/docs/carrier.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/pipeline/inspect/tests/carrier.rs")]
        /// ```
        pub mod inspect {}
    }

    /// Safety properties of the unsafe-mode and `no_unsafe` codegen
    /// paths — the regression guards behind every safety claim in the
    /// crate-level `Safety` section.
    pub mod safety {
        /// A failing fallible setter, a failing fallible overrider,
        /// and an async setter dropped mid-`await` all release every
        /// previously-set field — no `ManuallyDrop` leak.
        ///
        #[doc = include_str!("../tests/safety/factory_no_leak/docs/tests.md")]
        ///
        /// # Test Cases
        ///
        #[doc = include_str!("../tests/safety/factory_no_leak/docs/fallible_setter_failure_drops_other_set_fields.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_leak/tests/fallible_setter_failure_drops_other_set_fields.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_leak/docs/fallible_overrider_failure_drops_other_set_fields.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_leak/tests/fallible_overrider_failure_drops_other_set_fields.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_leak/docs/async_setter_dropped_mid_await_drops_other_set_fields.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_leak/tests/async_setter_dropped_mid_await_drops_other_set_fields.rs")]
        /// ```
        ///
        /// ## Shared types and setup
        ///
        #[doc = include_str!("../tests/safety/factory_no_leak/docs/error.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_leak/tests/error.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_leak/docs/bookkeeping.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_leak/tests/bookkeeping.rs")]
        /// ```
        ///
        pub mod factory_no_leak {}

        /// Panicking destructors and panicking `default = …` expressions
        /// must not leak the bag's other set fields. Pinned across the
        /// bag's `Drop`, `finalize`'s default branch, and the
        /// `override`/`drop_<field>` paths.
        ///
        #[doc = include_str!("../tests/safety/factory_panic_safety/docs/tests.md")]
        ///
        /// ## Test Cases
        ///
        #[doc = include_str!("../tests/safety/factory_panic_safety/docs/panic_in_drop_still_drops_subsequent_fields.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_panic_safety/tests/panic_in_drop_still_drops_subsequent_fields.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_panic_safety/docs/panic_in_default_expr_during_finalize_drops_already_read_fields.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_panic_safety/tests/panic_in_default_expr_during_finalize_drops_already_read_fields.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_panic_safety/docs/panic_in_old_value_drop_during_override_drops_other_fields.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_panic_safety/tests/panic_in_old_value_drop_during_override_drops_other_fields.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_panic_safety/docs/panic_in_old_value_drop_during_remove_drops_other_fields.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_panic_safety/tests/panic_in_old_value_drop_during_remove_drops_other_fields.rs")]
        /// ```
        ///
        /// ## Shared types and setup
        ///
        #[doc = include_str!("../tests/safety/factory_panic_safety/docs/bookkeeping.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_panic_safety/tests/bookkeeping.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_panic_safety/docs/panicky_drop.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_panic_safety/tests/panicky_drop.rs")]
        /// ```
        pub mod factory_panic_safety {}

        /// Macro internals are prefixed with `__tsh_` so a struct with
        /// fields named `_markers`, `this`, `__field_value`,
        /// `__old_field`, or `__new_bag` compiles cleanly. Default
        /// expressions still resolve free functions in the user's scope.
        ///
        #[doc = include_str!("../tests/safety/factory_hygiene/docs/tests.md")]
        ///
        /// # Test Cases
        ///
        #[doc = include_str!("../tests/safety/factory_hygiene/docs/struct_with_field_names_matching_macro_internals_compiles.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_hygiene/tests/struct_with_field_names_matching_macro_internals_compiles.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_hygiene/docs/default_expression_can_call_user_scope_function.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_hygiene/tests/default_expression_can_call_user_scope_function.rs")]
        /// ```
        pub mod factory_hygiene {}

        /// Bags with zero, one, and many flag generics all round-trip
        /// through `finalize`. The trailing-comma `PhantomData<(F,)>`
        /// marker preserves `Send`/`Sync` auto-trait forwarding for the
        /// singleton case (without it, the marker would collapse to
        /// `PhantomData<F>` and silently change variance).
        ///
        #[doc = include_str!("../tests/safety/factory_phantom_shape/docs/tests.md")]
        ///
        /// ## Test Cases
        ///
        #[doc = include_str!("../tests/safety/factory_phantom_shape/docs/all_internal_struct_finalizes_without_flag_generics.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_phantom_shape/tests/all_internal_struct_finalizes_without_flag_generics.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_phantom_shape/docs/singleton_flag_struct_round_trips.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_phantom_shape/tests/singleton_flag_struct_round_trips.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_phantom_shape/docs/one_flag_bag_is_send_and_sync_when_field_is.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_phantom_shape/tests/one_flag_bag_is_send_and_sync_when_field_is.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_phantom_shape/docs/many_flag_struct_round_trips.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_phantom_shape/tests/many_flag_struct_round_trips.rs")]
        /// ```
        pub mod factory_phantom_shape {}

        /// Parallel coverage suite for the `no_unsafe`-mode codegen
        /// path — every safety guarantee from the unsafe-mode tests
        /// holds in safe mode without `MaybeUninit`.
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/tests.md")]
        ///
        /// ## Test Cases
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/build_in_order.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/build_in_order.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/build_in_arbitrary_order.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/build_in_arbitrary_order.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/default_helper_fills_in_optional.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/default_helper_fills_in_optional.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/finalize_uses_default_when_optional_unset.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/finalize_uses_default_when_optional_unset.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/getter_borrows_set_field.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/getter_borrows_set_field.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/empty_bag_dropped_does_not_touch_unset_fields.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/empty_bag_dropped_does_not_touch_unset_fields.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/partial_bag_dropped_drops_only_set_fields.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/partial_bag_dropped_drops_only_set_fields.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/fully_populated_bag_dropped_drops_all.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/fully_populated_bag_dropped_drops_all.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/finalize_does_not_double_drop.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/finalize_does_not_double_drop.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/drop_field_drops_value_once.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/drop_field_drops_value_once.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/drop_field_then_reset_doesnt_double_drop.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/drop_field_then_reset_doesnt_double_drop.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/override_drops_old_value.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/override_drops_old_value.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/finalize_uses_defaults_when_optional_no.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/finalize_uses_defaults_when_optional_no.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/finalize_keeps_explicit_values_when_optional_yes.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/finalize_keeps_explicit_values_when_optional_yes.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/finalize_mixes_set_and_default.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/finalize_mixes_set_and_default.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/custom_transformer_fn.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/custom_transformer_fn.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/fallible_transformer_success.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/fallible_transformer_success.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/fallible_transformer_failure.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/fallible_transformer_failure.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/fallible_setter_failure_drops_other_set_fields.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/fallible_setter_failure_drops_other_set_fields.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/fallible_overrider_failure_drops_other_set_fields.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/fallible_overrider_failure_drops_other_set_fields.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/async_setter_dropped_mid_await_drops_other_set_fields.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/async_setter_dropped_mid_await_drops_other_set_fields.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/internal_field_round_trips.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/internal_field_round_trips.rs")]
        /// ```
        ///
        /// ## Shared types and setup
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/error.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/error.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/bookkeeping.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/bookkeeping.rs")]
        /// ```
        ///
        #[doc = include_str!("../tests/safety/factory_no_unsafe/docs/async_helpers.md")]
        ///
        /// ```rust
        #[doc = include_str!("../tests/safety/factory_no_unsafe/tests/async_helpers.rs")]
        /// ```
        pub mod factory_no_unsafe {}
    }

    /// Compile-fail diagnostics — what users see when they misuse the
    /// macros. Each page pairs the trybuild source with the pinned
    /// `.stderr` snapshot.
    pub mod ui {
        /// `<field>_default()` requires `default = …` on the field.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/default_helper_without_default.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/default_helper_without_default.html"))]
        pub mod default_helper_without_default {}

        /// `default = …` and `async_fn` are mutually exclusive — defaults
        /// must be synchronous expressions.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/default_with_async.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/default_with_async.html"))]
        pub mod default_with_async {}

        /// `default = …` and `fallible` are mutually exclusive.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/default_with_fallible.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/default_with_fallible.html"))]
        pub mod default_with_fallible {}

        /// `async_fn` requires `setter = …`.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/factory_async_without_setter.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/factory_async_without_setter.html"))]
        pub mod factory_async_without_setter {}

        /// `fallible` requires `setter = …`.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/factory_fallible_without_setter.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/factory_fallible_without_setter.html"))]
        pub mod factory_fallible_without_setter {}

        /// `#[factory(no_unsafe)]` is rejected when the `no_unsafe`
        /// Cargo feature is off.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/factory_no_unsafe_without_feature.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/factory_no_unsafe_without_feature.html"))]
        pub mod factory_no_unsafe_without_feature {}

        /// `#[derive(TypestateFactory)]` requires a struct.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/factory_on_enum.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/factory_on_enum.html"))]
        pub mod factory_on_enum {}

        /// `input = T` requires `setter = …`.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/input_without_setter.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/input_without_setter.html"))]
        pub mod input_without_setter {}

        /// `internal` fields don't get pipeline-arm methods even when
        /// `pipeline(carrier = …)` is set.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/internal_field_no_pipeline_arm.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/internal_field_no_pipeline_arm.html"))]
        pub mod internal_field_no_pipeline_arm {}

        /// `internal` fields cannot have a `setter = …` — they're
        /// positional on `new(…)` and have no setter.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/internal_with_setter.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/internal_with_setter.html"))]
        pub mod internal_with_setter {}

        /// `Pipeline::ctx` and `Pipeline::inner` are sealed — reading
        /// them from a downstream carrier newtype fails the privacy
        /// check, with a hint pointing at `ctx()` / `into_parts()`.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/pipeline_field_is_private.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/pipeline_field_is_private.html"))]
        pub mod pipeline_field_is_private {}

        /// `<Bag>Ready` does not auto-impl when a required flag is
        /// `No` — generic dispatch over `B: <Bag>Ready` is rejected for
        /// unfinalized bags.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/ready_trait_rejects_unset_required.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/ready_trait_rejects_unset_required.html"))]
        pub mod ready_trait_rejects_unset_required {}

        /// `#[transition]` requires `into = <Type>`.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/transition_without_into.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/transition_without_into.html"))]
        pub mod transition_without_into {}

        /// `breakpoint` is only meaningful on `async fn` bodies.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/transitions_breakpoint_on_sync.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/transitions_breakpoint_on_sync.html"))]
        pub mod transitions_breakpoint_on_sync {}

        /// `breakpoint` is a flag (no value) — `breakpoint = true` is rejected.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/transitions_breakpoint_with_value.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/transitions_breakpoint_with_value.html"))]
        pub mod transitions_breakpoint_with_value {}

        /// A `#[transition]` body's first parameter must be `state: <State>`.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/transitions_first_param_not_state.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/transitions_first_param_not_state.html"))]
        pub mod transitions_first_param_not_state {}

        /// `#[transitions]` only decorates inherent `impl` blocks — not
        /// trait `impl`s.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/transitions_on_trait_impl.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/transitions_on_trait_impl.html"))]
        pub mod transitions_on_trait_impl {}

        /// `#[transitions]` requires the carrier type to implement
        /// `Pipelined<'a>`.
        ///
        /// # Source
        ///
        /// ```rust,ignore
        #[doc = include_str!("../tests/ui/stable/transitions_without_pipelined_impl.rs")]
        /// ```
        ///
        /// # Expected diagnostic
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/diagnostics/transitions_without_pipelined_impl.html"))]
        pub mod transitions_without_pipelined_impl {}
    }
}

/// Declare a typestate carrier in one line: emits the newtype struct, its
/// `where M: Mode<…>` clause, the [`Pipelined`] impl, and the
/// [`IntoFuture`](core::future::IntoFuture) forwarding for `InFlight` mode.
///
/// ```no_run
/// # pub struct Client;
/// # #[derive(Debug)] pub struct AuthoringError;
/// typestate_pipeline::pipelined!(pub Author, ctx = Client, error = AuthoringError);
/// // optional: tag = MyTag (default `()`)
/// ```
///
/// Expands to:
///
/// ```text
/// pub struct Author<'a, S, M = Resolved>(Pipeline<'a, Client, (), S, AuthoringError, M>)
/// where M: Mode<'a, S, AuthoringError>;
/// // + Pipelined<'a> impl + IntoFuture forwarding
/// ```
///
/// Use [`impl_pipelined!`] when you need to hand-write the struct (custom
/// derives, extra generics like `<'a, K: Kind, S, M>`, etc.).
#[doc(alias = "carrier")]
#[doc(alias = "newtype")]
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
/// ```no_run
/// # struct Client;
/// # #[derive(Debug)] struct AuthoringError;
/// use typestate_pipeline::{Mode, Pipeline, Resolved};
///
/// struct Author<'a, S, M = Resolved>(Pipeline<'a, Client, (), S, AuthoringError, M>)
/// where M: Mode<'a, S, AuthoringError>;
///
/// typestate_pipeline::impl_pipelined!(Author, ctx = Client, error = AuthoringError);
/// ```
///
/// For the common case where you do not need to customize the struct, use
/// [`pipelined!`] which emits both in one line.
#[doc(alias = "carrier")]
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
