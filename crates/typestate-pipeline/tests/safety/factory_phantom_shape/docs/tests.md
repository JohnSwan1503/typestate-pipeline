# Phantom-marker tuple shape regression tests

The bag's `PhantomData<( F1, F2, … )>` marker is emitted with a
trailing comma after *every* element, including the singleton case.
That trailing comma is load-bearing:

| flag count | emitted | what it means |
|------------|---------|---------------|
| 0          | `PhantomData<()>` | unit tuple — empty marker |
| 1          | `PhantomData<(F,)>` | **singleton tuple** |
| many       | `PhantomData<(F1, F2, …)>` | normal tuple |

The singleton case is the dangerous one. `PhantomData<(F)>` (without
the trailing comma) is *not* a singleton tuple; parentheses around a
type just group, so it parses as `PhantomData<F>` and silently changes
the bag's variance and auto-trait inheritance. Adding the trailing
comma forces tuple parsing.

This file pins:

- The zero-flag case — every field is `internal`, so no flag generic
  list at all. The bag has `PhantomData<()>` and must compile +
  finalize cleanly.
- The one-flag case — the singleton `PhantomData<(F,)>` shape compiles
  and round-trips through finalize.
- The one-flag bag inherits `Send`/`Sync` via tuple auto-trait
  forwarding (matching the many-flag case). Without the trailing
  comma, this would compile too — but it would be inheriting
  `Send`/`Sync` from a bare `F`, which is a different reasoning path.
- The many-flag case — control case, three flags.

A future macro refactor that drops the trailing comma will surface as
a regression here rather than as a subtle variance / auto-trait shift
nobody notices.
