### Error type

`BadInput::Empty` — single-variant error reused by every fallible
path in the suite. Tests don't care about the error's identity;
they only need it to satisfy the carrier's `Pipelined::Error` slot
and to be `match`-able for the failure-path assertions.
