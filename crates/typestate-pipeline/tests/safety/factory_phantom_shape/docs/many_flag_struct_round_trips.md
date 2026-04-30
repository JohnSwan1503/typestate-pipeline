## Many-Flag Round-Trip

**Invariant.** Three-or-more-flag bags compile and round-trip through
`new` → setter chain → `finalize()`. This is the **control case** for
the singleton-tuple proof above.

**Failure mode this guards.** Nothing exotic — this test is the
control. Its job is to confirm that the multi-flag path (which is the
common case in real code) still works *while we're poking at the
singleton edge*. If a refactor accidentally broke the many-flag case
in pursuit of fixing the singleton, this test catches it.

**Setup.** A `struct ThreeFlags { a: u32, b: u32, c: u32 }` with all
three required.

**Assertion.** `finalize()` produces `(1, 2, 3)` after the setter
chain.

### many_flag_struct_round_trips
