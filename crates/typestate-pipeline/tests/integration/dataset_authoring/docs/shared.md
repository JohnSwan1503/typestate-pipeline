### Shared infrastructure

Re-exports the dataset-authoring example's primitives (`Client`,
`Namespace`, `Name`, `NetworkId`, `Reference`, `TableName`, `Version`)
so the per-test files can `use shared::*` without dragging in the full
crate path each time.

The `ns(s)` / `nm(s)` helpers exist purely to keep the test bodies
readable — `ns("eth")` instead of `Namespace("eth".to_owned())` — and
have no semantic content.
