# `<BagName>Ready` companion trait tests

Every `#[derive(TypestateFactory)]` also emits a companion trait
`<BagName>Ready` that auto-impls on every flag combination matching
`finalize()`'s bounds. The trait lets downstream code write
`where B: <BagName>Ready` instead of spelling out the entire flag
tuple — a generic "any finalize-callable bag" bound.

The trait method is named `finalize` — same name as the inherent —
so user code can call `bag.finalize()` whether `bag` is a concrete
flag tuple (resolves to the inherent) or a generic `B: <Bag>Ready`
(resolves to this trait method). The auto-impl body uses
fully-qualified path syntax (`<Bag<…>>::finalize(self)`) to dispatch
to the inherent rather than recurse into itself — inherent items take
precedence in path-qualified resolution.

This suite pins:

- The trait is implemented when every required field's flag is
  `Yes`, even if optional-with-default fields are still `No`.
- The trait still applies when the optional fields are explicitly
  set.
- Dispatching via the trait method produces the same value as
  calling the inherent `finalize()` directly — the trait impl body
  cannot silently diverge.

The companion *negative* assertion (a partially-set bag must NOT
implement the trait) lives in
[`tests::ui::ready_trait_rejects_unset_required`](../../ui/ready_trait_rejects_unset_required/index.html).
