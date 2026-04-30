### Error type

`AppError::Empty(field)` is the only error variant the suite
exercises — produced by the `trim_label` transformer when its
input trims to empty. The static-string slot lets tests pin the
exact field that triggered the failure.
