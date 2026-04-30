### Pipeline arm: setters on the carrier

Every recipe so far has built the bag *standalone*: callers
invoke setters on the bag, finalize, and only then hand the
result somewhere. That's fine when the bag is a configuration
struct assembled outside any phase machine. When the bag *is*
the phase the carrier sits in — when "build the bag" and
"advance the carrier" want to be the same fluent chain — the
unwrap-and-rewrap dance becomes friction.

`#[factory(pipeline(carrier = MyCarrier))]` instructs the derive
to emit setters, removers, overriders, default helpers, and
getters *also* on the user's `Pipeline`-newtype carrier. Each
method is emitted twice:

- on `Carrier<'a, Bag<…>, Resolved>`: takes the carrier, returns
  the carrier with the new bag state.
- on `Carrier<'a, Bag<…>, InFlight>`: takes the in-flight
  carrier, returns the in-flight carrier with the new bag state.
  Fallible methods fold the `Result` into the pending future,
  so the chain keeps reading flat at the call site.

Getters on the carrier are Resolved-only — there's no
synchronous read of an in-flight state without driving the
future first.

The codegen reaches into the carrier's tuple-struct internals,
which has a practical consequence: the bag definition has to be
colocated with its carrier (same module) so the visibility
works. When the bag is built independently and *handed in* to a
transition instead, see
[Factory in pipeline](#factory-in-pipeline-bag-built-outside-the-carrier)
in the combinations section.
