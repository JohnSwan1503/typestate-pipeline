Diff from the **Minimal** baseline above:

```rust,ignore
impl<F1> UserFactory<F1, No> {
    pub fn with_age(self, val: u32) -> UserFactory<F1, Yes>;  // optional → with_
    pub fn age_default(self)        -> UserFactory<F1, Yes>;  // default helper
}

// finalize is callable on EITHER state of `age`'s flag:
impl<F2: Satisfiable> UserFactory<Yes, F2> {              // unsafe-mode bound
    pub fn finalize(self) -> User;
}
// (safe mode uses `F2: Storage<u32>` instead of `Satisfiable`.)
```
