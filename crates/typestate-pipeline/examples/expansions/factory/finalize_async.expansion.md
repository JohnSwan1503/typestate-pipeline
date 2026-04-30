Diff from the **Minimal** baseline above:

```rust,ignore
impl UserFactory<Yes> {
    pub async fn finalize_async(self) -> Result<ConfirmedUser, BadInput>;
    //                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //   omit `error = …` to drop the `Result` wrapper:
    //   pub async fn finalize_async(self) -> ConfirmedUser;
}
```
