## Callable User Scope Functions 

**Invariant.** A `default = expr` attribute is interpreted in the
*user's* scope. Free functions, locals, type aliases, and consts that
are visible at the `#[derive(TypestateFactory)]` site must be reachable
from inside the default expression.

**Failure mode this guards.** The hygiene rename was a chance to
over-isolate the default expression's environment — for example by
emitting it inside a separate module or behind a synthetic identifier
that masked everything except the macro's own bindings. That would
make `default = my_helper()` fail to resolve `my_helper`, breaking
every existing user that relies on user-scope visibility.

The test pins the converse: a `default = user_helper()` that calls a
free function declared next to the struct still compiles and produces
the helper's return value at finalize time.

**Setup.** A free `fn user_helper() -> u32` is declared in the test's
module. `DefaultUsesUserScope` has a `name: String` (required) and an
`answer: u32` field with `default = user_helper()`. The bag is
finalized without setting `answer`, exercising the default branch.

**Assertion.** `s.answer == 42` (the helper's return). If the hygiene
rename had over-isolated the expression's scope, this would not
compile.

### default_expression_can_call_user_scope_function
