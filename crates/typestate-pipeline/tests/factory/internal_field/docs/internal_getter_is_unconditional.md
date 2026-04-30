## Internal Getter

**Invariant.** The internal field's getter (`bag.namespace()`) has
no `Yes`-flag bound — it's callable on the all-`No` bag, the
all-`Yes` bag, and every shape in between. Internal fields aren't
part of the typestate, so their accessor doesn't need to gate on
typestate.

**Failure mode this guards.** A buggy codegen could emit the
internal getter on a `Yes`-bound impl (treating it like a regular
field), making it inaccessible until at least one flag advances —
defeating the "constant-from-construction" semantics.

**Setup.** Brand-new bag from `JobFactory::new("op".to_owned())`.
Don't call any setter. Call `.namespace()` immediately.

**Assertion.** `bag.namespace() == "op"`.

### internal_getter_is_unconditional
