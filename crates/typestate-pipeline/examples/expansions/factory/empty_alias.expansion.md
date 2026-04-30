Addition to the **Minimal** baseline above:

```rust,ignore
type UserFactoryEmpty = UserFactory<No, No>;
```

The alias names the all-`No` flag-tuple shape — the bag returned by
`UserFactory::new(…)` before any setters run. Internal fields don't
appear in the flag-generic list, so the alias's tuple length equals
the count of non-internal fields.
