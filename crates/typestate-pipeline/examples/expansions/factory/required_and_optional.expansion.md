Diff from the **Minimal** baseline above:

```rust,ignore
impl<F2> UserFactory<No, F2> {
    pub fn name(self, val: String)         -> UserFactory<Yes, F2>;
}
impl<F1> UserFactory<F1, No> {
    pub fn with_nickname(self, val: String) -> UserFactory<F1, Yes>;
}
// finalize still binds both flags to `Yes`.
impl UserFactory<Yes, Yes> { pub fn finalize(self) -> User; }
```
