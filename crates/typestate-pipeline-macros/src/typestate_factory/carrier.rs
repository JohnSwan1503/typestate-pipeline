use crate::typestate_factory::spec::PipelineSpec;

/// Selects the surface a code-gen helper should emit for: the standalone
/// bag, or the user's `Pipelined<'a>` carrier.
#[derive(Clone, Copy)]
pub enum Carrier<'a> {
    Standalone,
    Pipeline(&'a PipelineSpec),
}
