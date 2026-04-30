```rust,ignore
impl<'a> Author<'a, Versioned, Resolved> {
    pub fn with_parallelism(self, parallelism: u16)
        -> Author<'a, JobConfigured, Resolved>;
}
impl<'a> Author<'a, Versioned, InFlight>
where
    Versioned: Send + 'a,
    JobConfigured: Send + 'a,
{
    pub fn with_parallelism(self, parallelism: u16)
        -> Author<'a, JobConfigured, InFlight>;
}
```
