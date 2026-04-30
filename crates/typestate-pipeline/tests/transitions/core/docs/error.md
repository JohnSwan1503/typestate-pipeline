### Error type

`TestError::Invalid(&'static str)` is a typed validation error
with a static-string slot so the failing-path tests can assert on
the exact message (catches regressions where the codegen swallows
which variant or message a transition body returned).
