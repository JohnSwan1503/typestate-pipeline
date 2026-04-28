//! Cross-feature integration scenarios: factory bags driving a `Pipeline`
//! carrier, `#[transitions]` plus factory in the same chain, and the
//! end-to-end dataset-authoring example.

#[path = "integration/dataset_authoring.rs"]
mod dataset_authoring;
#[path = "integration/factory_in_pipeline.rs"]
mod factory_in_pipeline;
#[path = "integration/factory_pipeline.rs"]
mod factory_pipeline;
