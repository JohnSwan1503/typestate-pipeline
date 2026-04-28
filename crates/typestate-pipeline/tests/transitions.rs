//! Feature coverage for `#[transitions]`: body shapes (sync/async ×
//! infallible/fallible), attribute forwarding onto the generated impls,
//! and destination-type resolution via `Pipelined`.

#[path = "transitions/attr_forwarding.rs"]
mod attr_forwarding;
#[path = "transitions/core.rs"]
mod core;
#[path = "transitions/via_pipelined.rs"]
mod via_pipelined;
