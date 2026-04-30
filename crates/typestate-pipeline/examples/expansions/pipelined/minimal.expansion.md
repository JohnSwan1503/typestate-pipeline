```rust,ignore
pub struct Author<'a, S, M = Resolved>(
    Pipeline<'a, Hub, (), S, AppError, M>,
)
where M: Mode<'a, S, AppError>;

impl<'a, S: 'a, M> Pipelined<'a> for Author<'a, S, M>
where M: Mode<'a, S, AppError>,
{
    type Ctx = Hub;
    type Error = AppError;
    type Tag = ();
    type State = S;
    type Mode = M;
    type Resolved<NS: 'a> = Author<'a, NS, Resolved>;
    type InFlight<NS: Send + 'a> = Author<'a, NS, InFlight>;
}

impl<'a, S> IntoFuture for Author<'a, S, InFlight>
where S: Send + 'a, AppError: Send + 'a, Hub: Sync + 'a,
{
    type Output = Result<Author<'a, S, Resolved>, AppError>;
    type IntoFuture = BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture { ... }
}

// `inspect` is documented in `inspect_combinator`.
```
