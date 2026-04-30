Diff from the **Minimal** baseline above:

```rust,ignore
// No flag generic for `namespace`; it lives in the struct as plain T.
struct JobFactory<F1 = No, F2 = No> { /* private */ }

impl JobFactory<No, No> {
    pub fn new(namespace: String) -> Self;   // positional
}
impl<F1, F2> JobFactory<F1, F2> {
    pub fn namespace(&self) -> &String;     // unconditional getter
}
// (no `fn namespace(self, …)` setter, no `drop_namespace`, no
//  `override_namespace`, no `namespace_default`.)
```
