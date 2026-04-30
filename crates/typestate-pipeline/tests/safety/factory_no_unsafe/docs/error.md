### Error types

Two minimal `std::error::Error` implementations reused across the
fallible-setter / overrider / transformer tests.

`Reject` is the plain "rejected" sentinel — no payload, no
`Display` data, just a marker that some path returned `Err`.

`ValidationError(&'static str)` carries a static message slot so the
fallible-transformer tests can pin the exact message in their
assertions (catches regressions where the codegen swallows the
specific error variant).
