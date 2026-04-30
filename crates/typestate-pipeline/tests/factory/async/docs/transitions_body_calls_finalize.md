## `finalize()` In Transition Body

**Invariant.** A `#[transitions]` impl whose `Self` type is a
fully-set bag (`OrderFactory<Yes, Yes>`) can call `finalize()`
inside its body and use the resulting struct to build the next
phase's value. This is the most flexible finalize integration —
more so than `finalize_async`, because the body has access to the
carrier's context.

**Failure mode this guards.** Two things have to work:

1. **The transition's bound on the bag's flag tuple.** Calling
   `book()` on an under-set bag (`OrderFactory<No, Yes>` etc.)
   must NOT compile. Only `(Yes, Yes)` should typecheck.
2. **`finalize()` callable inside an async transition body.** The
   bag is consumed normally; the resulting struct is moved into
   the new phase value. If the transition's `state` parameter
   was somehow restricted (e.g. behind a borrow), `finalize()`
   couldn't be called.

**Setup.** Pipeline opens with `empty_order(&hub)`. Setter chain:
`sku("widget").quantity(3)` advances both flags to `Yes`. Then
`book()` (an async-deferred transition on the fully-set bag)
finalizes inside its body and produces a `Booked`. Terminal
`.await?`.

**Assertion.** The booked carrier's state has `sku == "widget"`,
`quantity == 3` (both came through the bag's `finalize`),
`receipt_id == 1234` (set by the transition body).

### transitions_body_calls_finalize
