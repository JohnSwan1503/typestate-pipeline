Diff from the **Minimal** baseline above:

```rust,ignore
impl<F2> UserFactory<Yes, F2> {
    pub fn drop_name(self) -> UserFactory<No, F2>;  // Yes -> No
}
```
