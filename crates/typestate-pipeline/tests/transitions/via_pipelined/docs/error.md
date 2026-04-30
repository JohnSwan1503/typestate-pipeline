### Error type

`AppError::Bad(&'static str)` is the carrier's error type, the
type the suite's `#[transitions]` impls read off
`<Self as Pipelined<'a>>::Error` (since they all omit the explicit
`error =` arg). The static-string slot lets the failure-path tests
pin the exact message.
