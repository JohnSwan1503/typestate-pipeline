### Error type

`DummyError` is a stub `std::error::Error`. The smoke tests never
exercise an error path; the type only exists to satisfy the
carriers' `Pipelined::Error` slot.
