## Defaults at `finalize`

**Invariant.** A bag with multiple optional-with-default fields, none
of which were explicitly set, finalizes with each field equal to its
declared default expression's value.

**Failure mode this guards.** Same shape as
[`finalize_uses_default_when_optional_unset`](../finalize_uses_default_when_optional_unset/index.html)
but exercises *multiple* default branches at once, plus a non-trivial
default expression (`String::to_owned()` produces a heap-allocated
default value). Catches regressions where the dispatch worked for one
field but broke when the macro expanded multiple `Storage::finalize_or`
calls back-to-back.

**Setup.** `Configurable` with `name` (required), `parallelism`
(default 8), `url` (default `"https://default.example".to_owned()`).
Only `name` is set.

**Assertion.** All three default values appear in the finalized
struct.

### finalize_uses_defaults_when_optional_no
