Addition to standalone setters:

```rust,ignore
// Resolved-mode setter on the carrier.
impl<'a, F2> Author<'a, OrderFactory<No, F2>, Resolved> {
    pub fn sku(self, val: String) -> Author<'a, OrderFactory<Yes, F2>, Resolved>;
}

// InFlight-mode setter on the carrier (Send/Sync-bounded).
impl<'a, F2> Author<'a, OrderFactory<No, F2>, InFlight>
where
    OrderFactory<No, F2>: Send + 'a,
    OrderFactory<Yes, F2>: Send + 'a,
{
    pub fn sku(self, val: String) -> Author<'a, OrderFactory<Yes, F2>, InFlight>;
}

// … same shape for `with_<f>`, `<f>_default`, `drop_<f>`, `override_<f>`.
// Resolved-only getter:
impl<'a, F2> Author<'a, OrderFactory<Yes, F2>, Resolved> {
    pub fn sku(&self) -> &String;
}
```
