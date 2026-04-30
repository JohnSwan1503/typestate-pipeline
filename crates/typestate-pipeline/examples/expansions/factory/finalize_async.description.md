### Async finalize: post-finalize hooks

`finalize()` is the bag's terminal step — it consumes the
typestate machinery and hands back the original struct. For
some workflows that's not the actual finish line: the assembled
struct still needs to be confirmed against a remote service,
signed, persisted, or otherwise routed through one async hop
before the caller can really say "done."

`#[factory(finalize_async(via = my_fn, into = Target, error =
E?))]` emits an `async fn finalize_async()` callable on the same
bag shapes as `finalize()`. The body is exactly
`my_fn(self.finalize()).await`, so the hook receives the
assembled raw struct and returns whatever it produces — the
inherent `finalize()` is *not* replaced; both coexist, and the
caller picks per call site.

The `into = …` and `error = …` knobs control the return type.
Drop `error = …` to skip the `Result` wrapper for an
infallible hook.
