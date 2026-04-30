## `finalize` No Double-Drop

**Invariant.** `finalize()` consumes the bag and produces the original
struct, moving each `Counted` exactly once. The bag's residual
storage (just the `_markers: PhantomData` and any `()` `No`-slots) is
not allowed to re-run `Drop` on already-moved values.

**Failure mode this guards.** In safe mode, `finalize` partial-moves
the `T`-slot fields out of the sister-struct, leaving Rust's compiler
to handle the resulting partial-move state cleanly. If safe mode
*also* emitted a manual `Drop` impl that ran on a partially-moved
struct, Rust's drop checker would complain (or worse, run `T::drop`
on a moved value — UB).

The whole point of the safe-mode codegen path is that there's *no*
manual `Drop` impl, so the compiler's auto-derived `Drop` plus the
partial-move analysis is what guarantees correctness.

**Setup.** `DropTrace` fully populated. `finalize()` consumes the bag
and we hold the resulting `DropTrace` in a local until end-of-scope.

**Assertion.** Between finalize and end-of-scope, `alive() ==
baseline + 2` — both `Counted`s moved into the result, both still
alive. After end-of-scope, `alive() == baseline` — both dropped
exactly once via the result's auto-Drop. No double-drop.

### finalize_does_not_double_drop
