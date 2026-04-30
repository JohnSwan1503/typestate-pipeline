Diff from the **Minimal** baseline above:

```rust,ignore
pub struct Author<'a, S, M = Resolved>(
    Pipeline<'a, Hub, RawKind, S, AppError, M>,
)                          //  ^^^^^^^ tag slot
where M: Mode<'a, S, AppError>;

impl<...> Pipelined<'a> for Author<'a, S, M> {
    type Tag = RawKind;        // <-- the only Pipelined associated-type diff
    ...
}
```
