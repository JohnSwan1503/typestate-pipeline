### Standalone bags + finalize_async target

The non-pipeline-arm `TypestateFactory` derives:

- `UserProfile` — async setter coverage. `name` uses an
  async-non-fallible transformer (`normalize_name_async`); `email`
  uses an async-fallible transformer (`validate_email_async`).
- `User` + `ConfirmedUser` + `confirm_user` — `User` declares
  `finalize_async(via = confirm_user, into = ConfirmedUser, error = BadInput)`.
  The hook runs inside the async finalize and produces the
  post-confirm value.

These bags don't reference the carrier — they exist for the
standalone async tests. The Pipeline-integrated bag (`Order`)
lives next to its carrier in [`carrier`](../carrier/index.html)
because the derive's pipeline arm reaches into the carrier's
internals.
