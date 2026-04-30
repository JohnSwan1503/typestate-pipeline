### Sync fallible: errors at the call site

Adding fallibility to a sync transition is a one-character
change in the body — return `Result<Next, E>` instead of
`Next` — but the two emitted arms diverge in interesting ways.

- The **Resolved arm** returns `Result<Carrier<Next, Resolved>, E>`.
  The error is right at the call site; no `.await` involved,
  the caller writes `?` immediately.
- The **InFlight arm** *folds* the body's `Result` into the
  pending future. There's no `Result` at the call site —
  attempting one would break the chain — and the error
  surfaces at the chain's terminal `.await?` instead.

That asymmetry is the cost of folding. The Resolved arm wants
the error eagerly because there's no future to fold into; the
InFlight arm wants the error lazily because the future is the
whole point. Both shapes are ergonomic in their own context.

The error type comes from `#[transitions(error = E)]` on the
impl block. Fallible transitions compile only when an error
type is declared — the macro can't infer it from the body
because every transition in the impl block has to fold into the
same `E`.
