### Minimal: every field required

Start with a struct whose fields are both `required`. The derive
emits four pieces of API on top of it — and that's all:

- `<Name>Factory::new()` constructs an empty bag where every flag
  is `No`.
- One setter per field consumes `self` and flips that field's
  flag from `No` to `Yes`.
- A getter for each field becomes callable once the flag is `Yes`.
- `finalize()` consumes the bag back into the original struct
  once every required flag is `Yes`.

Each setter signature bounds *one* flag at a time and leaves
the others free, so the same chain compiles regardless of the
order the caller fills fields in. The constraint that
`finalize()` requires *both* flags `Yes` lives in a single impl
block — the only place the compiler has to enforce "every
required field is set."

The companion `<BagName>Ready` trait at the bottom of the sketch
is covered later, in [The Ready trait](#the-ready-trait-generic-over-finalize-callable).
