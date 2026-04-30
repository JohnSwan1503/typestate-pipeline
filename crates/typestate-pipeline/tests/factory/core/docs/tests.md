# `#[derive(TypestateFactory)]` core feature coverage

Standalone tests for the unsafe-mode `TypestateFactory` codegen.
Organized by feature: construction & getters, Drop semantics on
unset/partial/fully-set/finalized bags, removable, overridable,
defaults / conditional finalize, custom names, and custom
transformers (sync infallible + sync fallible).

Pipeline-integrated behavior lives in
[`tests::integration::factory_pipeline`](../../integration/factory_pipeline/index.html);
async setters and `finalize_async` live in
[`tests::factory::async_setters`](../async_setters/index.html);
leak-on-failure regressions live in
[`tests::safety::factory_no_leak`](../../safety/factory_no_leak/index.html).

The `no_unsafe`-mode parallel of this exact suite lives in
[`tests::safety::factory_no_unsafe`](../../safety/factory_no_unsafe/index.html);
when the same test name appears in both, the assertion is
identical — only the underlying codegen path differs.
