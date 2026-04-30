### Phase-state types

Three plain owned structs for the carrier's
`Drafted -> Tagged -> Deployed` chain. `tag` and `job_id` come
from `Hub::allocate()`, so the assertions can pin specific values
(`tag == 1`, `job_id == 2`).
