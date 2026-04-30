### Phase-state types

The four phase-state types the chain walks through, in order:

- `Registered` — initial state with `name` + `manifest_hash`.
- `Versioned` — `Registered` plus a `version` number.
- `JobConfigured` — `Versioned` plus `parallelism` + a `verified`
  flag that the sync-fallible `validate_and_finalize` flips to
  `true` on success.
- `Deployed` — final state with `name` + `version` + `job_id`
  (allocated by the `deploy` transition).

Each is a plain owned struct — no typestate machinery in these
types themselves; the typestate lives on the `Author` carrier.
