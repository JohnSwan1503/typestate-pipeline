## Panic In `default = …`

**Invariant.** When `finalize()` evaluates a `default = expr` thunk for
an unset optional and that expression panics, every field that was
*already* moved out of the bag's `MaybeUninit` storage must drop on
unwind. No leak.

**Failure mode this guards.** A naive `finalize()` interleaved
`assume_init_read` calls with the struct-literal expression that
constructs the output:

```text
RawUser {
    a: assume_init_read(self.a),   // moved out
    b: panicking_default(),         // panics here
    c: assume_init_read(self.c),    // never runs
}
```

If the default expression in `b`'s slot panicked, `c`'s read never
fired. Worse, the partially-constructed struct literal's `MaybeUninit`
temps lived inside the `ManuallyDrop`-wrapped `self`, which suppresses
Drop. The result: `a` was an owned local that auto-dropped, but `c`
was leaked.

The fix reads every initialized field into a stack local *before*
evaluating any `default = …` thunk:

```text
let __tsh_a = assume_init_read(self.a);
let __tsh_c = assume_init_read(self.c);
let __tsh_b = panicking_default();   // panic here unwinds with __tsh_a / __tsh_c live
RawUser { a: __tsh_a, b: __tsh_b, c: __tsh_c }
```

Both `a` and `c` are now owned locals, so the unwind drops them.

**Setup.** A bag with three fields — `a: Counted`, `b: u32` with
`default = panicking_default()` (a thunk that always panics), and
`c: Counted`. `a` and `c` are set; `b` stays unset so finalize must hit
the default branch. `alive() == 2`.

`bag.finalize()` is called inside `catch_unwind`. The default thunk
panics during the read sequence.

**Assertion.** After `catch_unwind` returns `Err`:

- `result.is_err()` confirms the default panicked.
- `alive() == 0` confirms `a` and `c` both auto-dropped on unwind.

### panic_in_default_expr_during_finalize_drops_already_read_fields
