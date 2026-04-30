Diff from the **Minimal** baseline above:

```rust,ignore
// `#[factory(name = ManifestBuilder)]` renames the bag type:
struct ManifestBuilder<F1 = No> { /* private */ }

// `#[field(name = shout_title)]` renames the setter:
impl ManifestBuilder<No> {
    pub fn shout_title(self, val: String) -> ManifestBuilder<Yes>;
}
// Getter still resolves under the field name (`title`), not the setter name.
impl ManifestBuilder<Yes> { pub fn title(&self) -> &String; }
```
