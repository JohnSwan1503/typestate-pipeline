### Error type

`AppError` is the carrier's error slot. The internal-field tests
never exercise an error path, so the enum has no variants — the
type only exists to satisfy the carrier's `Pipelined::Error`
projection.
