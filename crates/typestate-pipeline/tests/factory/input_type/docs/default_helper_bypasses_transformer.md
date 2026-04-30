## Default Helper Bypass

**Invariant.** When a field has both `default = expr` and
`input = T`, the `<field>_default()` helper writes the field's
storage type directly — bypassing the transformer that the regular
setter would otherwise apply. The default expression is typed as
the storage type, not the input type.

**Failure mode this guards.** Without the bypass, the
`worker_default()` helper would route through `wrap_some`, which
expects `String` as input — but `default = None` produces
`Option<String>`. The compile would fail because the transformer
can't accept the default's value, or it would silently coerce in
the wrong direction.

The fix in `gen_default_helper` was to emit the helper as a direct
field write rather than as `with_worker(default_expr)`. This test
locks in the bypass.

**Setup.** Same `Profile` bag. Chain:
`name("bob").worker_default()` — calls the helper, which writes
`None: Option<String>` straight into storage.

**Assertion.** `profile.worker == None`.

### default_helper_bypasses_transformer
