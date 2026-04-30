## `#[derive(TypestateFactory)]` — building values

The first macro turns a regular Rust struct into a *named-field
accumulator*: a fluent builder whose typestate tracks, in the
type itself, which fields have been set. Setters are callable
only against the unset side of a flag, getters only against the
set side, and `finalize()` compiles only when every `required`
flag is `Yes`. There is no runtime check anywhere in that chain
— the compiler is the only enforcer.

The recipes below cover every attribute the derive understands.
They sit on three independent axes:

- **Flag transitions.** `required` and `optional` set the entry
  side of the flag. `default` relaxes the finalize bound on the
  exit side. `removable` and `overridable` open reverse and
  in-place transitions on a flag that's already `Yes`.
- **Setter bodies.** What runs *inside* a setter when a value is
  being stored. `setter = my_fn` runs a transformer; `fallible`
  lets that transformer reject the input; `async_fn` lets it
  await; `input = T` lets the call-site type differ from storage.
- **Structural knobs.** Renames, the `<Bag>Ready` companion
  trait, the `<Bag>Empty` alias, the `pipeline(carrier = …)`
  arm that emits setters on a phase carrier, the `no_unsafe`
  codegen mode, and `#[field(internal)]` for fields locked at
  construction.

Each recipe layers a single feature on top of the previous one,
so the diff sketches stay short.
