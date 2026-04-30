### The Empty alias: naming the entry-side shape

The companion to [`<Bag>Ready`](#the-ready-trait-generic-over-finalize-callable). Where
`<Bag>Ready` hides the all-`Yes` flag tuple at the *exit* (so generic
code can accept "any finalize-callable bag"), `<Bag>Empty` hides the
all-`No` flag tuple at the *entry* (so a function returning a fresh
bag — most commonly a `#[transition]` body that opens the
configuration phase — doesn't have to spell out `<No, No, …>`).

For a bag with three non-internal fields:

```rust,ignore
#[derive(TypestateFactory)]
struct Settings {
    #[field(required)]   endpoint: String,
    #[field(default = 30)] timeout_secs: u32,
    #[field(default, removable, overridable)] label: String,
}
```

the macro emits, alongside `SettingsFactoryReady`:

```rust,ignore
type SettingsFactoryEmpty = SettingsFactory<No, No, No>;
```

The most common use is in a `#[transition]` that enters the
configuration phase from a prior state:

```rust,ignore
#[transition(into = SettingsFactoryEmpty)]
fn configure(state: Idle) -> SettingsFactoryEmpty {
    SettingsFactory::new()
}
```

`#[field(internal)]` fields don't appear in the flag-generic list,
so the alias's tuple length equals the count of non-internal fields.
A bag with zero non-internal fields gets `pub type <Bag>Empty =
<Bag>;` (no angle brackets) — usable but pointless, since there's no
flag tuple to shorthand for.
