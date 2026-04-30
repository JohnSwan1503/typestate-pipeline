Diff from the **Minimal** baseline above:

```rust,ignore
impl UserFactory<No> {
    pub fn name(self, val: String)
        -> Result<UserFactory<Yes>, ValidationError>
    { /* body: stores `require_nonempty(val)?` */ }
}
```
