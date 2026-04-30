```rust,ignore
impl<'a> Author<'a, Registered, Resolved> {
    pub async fn confirm_and_tag(self)
        -> Result<Author<'a, Versioned, Resolved>, AppError>;
    //                                  ^^^^^^^^   resolved, not InFlight
}
impl<'a> Author<'a, Registered, InFlight>
where /* Send + 'a bounds */
{
    pub async fn confirm_and_tag(self)
        -> Result<Author<'a, Versioned, Resolved>, AppError>;
    //                                  ^^^^^^^^   also a breakpoint
}
```
