### `PanickyDrop` — one-shot panicking destructor

Payload type whose `Drop` panics on its first invocation, then
behaves normally for any subsequent drops. The "first only"
behavior is enforced by `PANICKY_FUSE` (a one-shot atomic from the
[`bookkeeping`](#bookkeeping) module) — when the
override / remove tests construct a fresh `PanickyDrop` as the
new value, that value's eventual `Drop` is a no-op since the fuse
is already spent.

This is what lets the override / remove tests exercise panic
unwinding *once* without immediately tripping double-panic abort
when the unwind stack runs the new value's `Drop` too.
