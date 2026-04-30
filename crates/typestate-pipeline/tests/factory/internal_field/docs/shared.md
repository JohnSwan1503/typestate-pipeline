### Shared infrastructure

- `Hub` / `AppError` — context and error types for the carrier.
- `Author` — carrier declared via `pipelined!`. Used by the
  carrier-arm tests; the standalone tests just use the bag.
- `Job` — three-field bag exercising one `internal` field
  (`namespace`) and two regular fields (`parallelism` required,
  `verify` optional with default). The bag uses
  `pipeline(carrier = Author)` so it gets carrier-arm methods
  emitted automatically.
- `carrier(hub, namespace)` — convenience constructor wrapping
  `Author(Pipeline::resolved(hub, JobFactory::new(namespace)))`.
- `into_state()` accessor on the Resolved carrier so per-test
  files can finalize through the chain.
