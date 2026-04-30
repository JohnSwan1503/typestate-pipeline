```rust,ignore
impl<'a, S: 'a> Author<'a, S, Resolved> {
    pub fn inspect<F>(self, f: F) -> Self
    where F: FnOnce(&Self);
}
impl<'a, S> Author<'a, S, InFlight>
where S: Send + 'a, AppError: Send + 'a, Hub: Sync + 'a,
{
    pub fn inspect<F>(self, f: F) -> Self
    where F: FnOnce(&Author<'a, S, Resolved>) + Send + 'a;
}
```
