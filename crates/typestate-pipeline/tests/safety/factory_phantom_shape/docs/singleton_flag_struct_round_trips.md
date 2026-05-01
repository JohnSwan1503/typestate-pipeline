## Singleton-Flag Round-Trip

**Invariant.** A struct with exactly one non-internal field produces a
bag with one flag generic. The phantom marker is `PhantomData<(F,)>` —
a singleton tuple, with a trailing comma. `new` → `setter` →
`finalize()` must round-trip exactly like a multi-flag bag.

**Failure mode this guards.** Without the trailing comma, the macro
would emit `PhantomData<(F)>`, which Rust parses as `PhantomData<F>`
(the parens just group a type expression). The compile-pass / round-trip
check still works for either spelling, so this test is a *direct*
proof: the bag finalizes into the expected value, end of story. The
auto-trait test
([`one_flag_bag_is_send_and_sync_when_field_is`](#one_flag_bag_is_send_and_sync_when_field_is))
does the secondary check that the singleton tuple is still acting like
a tuple at the auto-trait level.

**Setup.** A `struct OneFlag { name: String }` deriving
`TypestateFactory`. The setter chain is `OneFlagFactory::new().name(...)`.

**Assertion.** `finalize()` returns `OneFlag` with `name == "hello"`.

### singleton_flag_struct_round_trips
