### Error type

`AppError` is the carrier's error slot — an uninhabited enum (the
suite never produces an error). It still implements `Display` and
`Error` because the `pipelined!` macro requires a real error type
in the `error =` slot.
