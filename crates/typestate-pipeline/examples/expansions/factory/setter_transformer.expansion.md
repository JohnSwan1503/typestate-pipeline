Diff from the **Minimal** baseline above:

```rust,ignore
impl UserFactory<No> {
    pub fn name(self, val: String) -> UserFactory<Yes> {
        // body stores `trim_name(val)`
    }
}
```

The setter signature is identical to the baseline; only the body
differs.
