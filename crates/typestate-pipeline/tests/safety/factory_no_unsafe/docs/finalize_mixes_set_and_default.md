## Mixed Explicit + Default

**Invariant.** A bag with some optional-with-default fields set and
others unset finalizes with each field's value coming from the
appropriate branch — explicit-set fields get their stored values,
unset fields get their declared defaults. The branches are picked
per-field, not per-bag.

**Failure mode this guards.** A subtle codegen regression could pin
the dispatch branch globally (e.g. "if any optional is set, take all
optionals from storage; otherwise all from defaults"). That would
work in the all-set or all-unset cases but corrupt mixed cases. This
test pins the per-field independence.

**Setup.** `Configurable` with `name` set and `with_parallelism(4)`
explicit, but `url` left unset.

**Assertion.** `parallelism == 4` (explicit) and `url ==
"https://default.example"` (default).

### finalize_mixes_set_and_default
