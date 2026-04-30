### Sync infallible: the simplest body shape

The simplest transition is a synchronous function that can't
fail: a `fn` returning a non-`Result`. Phase advancement that's
pure transformation — repackaging fields, computing derived
state, lifting one phase struct into another.

This is also where the two-arm scheme starts paying off, even
without any async involved. A sync transition has to compile
the same in a chain that's already been lifted to `InFlight`
by some earlier async link as it does at the head of a fresh
chain — otherwise mixing sync and async would split the call
site into "before any async happens" and "after," and every
sync step after the first async step would need a manual
`.await?` to drop back to `Resolved`. The macro emits both
arms so neither side has to know about the other:

- The **Resolved arm** applies the body inline and returns the
  carrier in `Resolved` mode for the next state. Sync, eager,
  no `.await` involved.
- The **InFlight arm** awaits the pending state, applies the
  body, and re-wraps the result as `InFlight`. The chain keeps
  folding into one terminal `.await?` — a sync transition
  doesn't break it.

Mixed sync-and-async chains then read flat: every link looks
like a method call regardless of which mode it's actually
operating in.
