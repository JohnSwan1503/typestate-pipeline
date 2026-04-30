Diff from the **Minimal** baseline above:

```rust,ignore
impl<F1> ProfileFactory<F1, No> {
    // Setter takes `String` (the input type), not `Option<String>`.
    pub fn with_worker(self, val: String) -> ProfileFactory<F1, Yes>;
    // Default helper writes the field type directly (`Option<String>`),
    // bypassing the transformer entirely.
    pub fn worker_default(self) -> ProfileFactory<F1, Yes>;
}
```
