```rust,ignore
impl<'a> Author<'a, JobConfigured, Resolved> {
    pub fn validate(self)
        -> Result<Author<'a, JobConfigured, Resolved>, AppError>;
}
impl<'a> Author<'a, JobConfigured, InFlight>
where /* Send + 'a bounds */
{
    pub fn validate(self) -> Author<'a, JobConfigured, InFlight>;
    //                       ^^^ Result is folded into the pending future
}
```
