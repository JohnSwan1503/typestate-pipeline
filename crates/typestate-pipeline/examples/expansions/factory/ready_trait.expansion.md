Addition to the **Minimal** baseline above:

```rust,ignore
trait UserFactoryReady: Sized {
    fn finalize(self) -> User;
}
impl<F1: Satisfied, F2: Satisfiable> UserFactoryReady       // unsafe-mode
    for UserFactory<F1, F2> { /* delegates to finalize() */ }
// (safe mode: `F1` is pinned to concrete `Yes`, `F2: Storage<u32>`.)
```
