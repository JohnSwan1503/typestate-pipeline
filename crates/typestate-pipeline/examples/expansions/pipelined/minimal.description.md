### Minimal: a carrier in one line

A single `pipelined!(Author, ctx = Hub, error = AppError)`
emits four pieces:

1. The newtype struct `pub struct Author<'a, S, M = Resolved>(...)`
   — the carrier itself.
2. The [`Pipelined<'a>`](crate::Pipelined) impl that projects
   `Ctx` / `Error` / `Tag` and the `Resolved<NS>` / `InFlight<NS>`
   GAT successors `#[transitions]` uses to compute the next
   phase's type.
3. An [`IntoFuture`](core::future::IntoFuture) impl on
   `Author<'a, S, InFlight>`, so awaiting an in-flight carrier
   yields a `Result<Author<'a, S, Resolved>, Error>`. This is
   what powers the deferred fold from the previous section.
4. A chainable `inspect(|c| …)` combinator on both modes (the
   subject of the next section).

The `pub` (or its absence) on the macro invocation propagates
to the struct. Drop the leading `pub` to scope the carrier
private to its module — usually the right call, because most
carriers exist to expose an entry-state factory and a
terminal-state getter at the module boundary, with the
intermediate phases hidden behind the carrier so callers can't
reach in and skip a step. Adding `pub` works too, but it
requires the ctx/error/tag types to also be `pub`, since the
generated `Pipelined` impl exposes them.
