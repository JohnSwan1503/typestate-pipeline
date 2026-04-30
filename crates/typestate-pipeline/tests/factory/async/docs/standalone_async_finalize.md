## Async Finalize

**Invariant.** `#[factory(finalize_async(via = my_fn, into = Target, error = E?))]`
emits an `async fn finalize_async()` on the bag. The body is
`my_fn(self.finalize()).await`, so the user's hook receives the
fully-built raw struct and produces the final value (with or
without an error wrapper).

The inherent `finalize()` still exists and produces the raw
struct; `finalize_async` is additive.

**Failure mode this guards.** The async finalize is a separate
codegen branch from the regular finalize. Possible regressions:

- The hook isn't called — the raw struct is returned directly,
  failing the `ConfirmedUser` type assertion.
- The hook is called but its `Result` is dropped — `expect` would
  fire on a wrong path.
- `finalize_async` shadows `finalize` instead of coexisting —
  calling `finalize()` would route through the hook (or fail to
  compile).

**Setup.** `User` bag with `finalize_async(via = confirm_user,
into = ConfirmedUser, error = BadInput)`. The test calls both
`finalize()` (returns raw `User`) and `finalize_async().await`
(returns `Result<ConfirmedUser, BadInput>`).

**Assertion.**

- Sync `finalize()` returns `raw.name == "Carol"`.
- `finalize_async().await.expect(...)` returns
  `ConfirmedUser { name: "Carol", confirmation_token: "token-for-Carol" }`.

### standalone_async_finalize
