// `Pipeline`'s `ctx` and `inner` fields are sealed: a downstream user
// cannot reach into them via `pipeline.ctx` / `pipeline.inner` to forge
// or replace the typestate carrier's contents.

use typestate_pipeline::Pipeline;

struct DummyCtx;

fn main() {
    let ctx = DummyCtx;
    let p: Pipeline<'_, DummyCtx, (), u32, ()> = Pipeline::resolved(&ctx, 7);
    let _ = p.ctx;
    let _ = p.inner;
}
