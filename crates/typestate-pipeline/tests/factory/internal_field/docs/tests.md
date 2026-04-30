# `#[field(internal)]` coverage

Internal fields are positional on `new(…)` and locked from then on.
They differ from regular flag-tracked fields in four observable
ways:

1. `Factory::new(…)` accepts the internal field as a positional
   argument; no setter chain is needed for it.
2. The bag's flag generic list does not include the internal field
   (a struct with two non-internal fields gets `Factory<F1, F2>`,
   not `Factory<F1, F2, F3>`).
3. No setter, remover, overrider, or default helper is emitted for
   internal fields.
4. The getter is unconditional — callable on any bag shape with no
   flag bound.

This suite pins each of those user-visible properties plus the
carrier-arm versions (where applicable). Negative properties (no
setter exists, no overrider exists, etc.) live in the trybuild ui
suite.
