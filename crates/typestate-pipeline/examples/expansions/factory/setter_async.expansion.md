Diff from the **Minimal** baseline above:

```rust,ignore
impl<F2> UserFactory<No, F2> {
    pub async fn name(self, val: String) -> UserFactory<Yes, F2>;
}
impl<F1> UserFactory<F1, No> {
    pub async fn email(self, val: String)
        -> Result<UserFactory<F1, Yes>, BadInput>;
}
```
