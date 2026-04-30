Diff from the **Minimal** baseline above:

```rust,ignore
impl UserFactory<Yes> {
    pub fn override_name(self, val: String) -> UserFactory<Yes>; // Yes -> Yes
}
```
