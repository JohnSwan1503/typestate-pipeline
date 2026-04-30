### Domain types

`Server` is the carrier's context — a stub external system that
hands out monotonically-increasing `job_id`s. `AtomicU64` so the
assertions can pin specific values (e.g. `job_id == 1`) without
locking.
