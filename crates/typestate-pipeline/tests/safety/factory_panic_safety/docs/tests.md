# Panic-mode regression tests

Panicking destructors and panicking `default = …` expressions must not
leak the bag's other set fields. The unsafe-mode codegen had three
distinct failure modes here before the per-field RAII / read-into-stack-temp
restructure:

1. **Bag's `Drop` impl.** A panic in field N's `T::drop` previously
   short-circuited the manual Drop body, leaking fields N+1..end. Now
   each field is read out into an owned `Option<T>` stack guard so the
   auto-drop's cleanup-on-panic still runs the remaining destructors.
2. **`finalize()`'s default-expression branch.** A panic in
   `#default_expr` previously left fields after it un-`ptr::read`'d
   inside the `ManuallyDrop` wrapper, leaking them. Now `finalize()`
   reads every initialized field into a stack local *before*
   evaluating any default thunk.
3. **`override_<field>` / `drop_<field>`.** A panic in the OLD value's
   `T::drop` (called in-body via `assume_init_drop`) previously leaked
   the other fields because the new bag hadn't been built yet. Now the
   old value is read into a stack temp and dropped at end-of-scope,
   *after* the new bag is constructed.

Each test uses `catch_unwind` to observe the live-counter delta after
the unwind settles. `Counted` is a generic alive-counter; `PanickyDrop`
panics on its first drop only (a one-shot via `PANICKY_FUSE`) so we
can probe the panic path without immediately tripping double-panic
abort when an additional `PanickyDrop` is also dropped on unwind.

Tests serialize against each other via `LOCK` because the per-binary
atomic counters can otherwise race across `cargo test`'s parallel
within-binary scheduling.
