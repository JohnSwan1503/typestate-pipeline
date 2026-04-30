```rust,ignore
impl<'a> Author<'a, Registered, Resolved> {
    pub fn tag_version(self, version: u32)
        -> Author<'a, Versioned, InFlight>;   // lift to InFlight
}
impl<'a> Author<'a, Registered, InFlight>
where /* Send + 'a bounds */
{
    pub fn tag_version(self, version: u32)
        -> Author<'a, Versioned, InFlight>;
}
```
