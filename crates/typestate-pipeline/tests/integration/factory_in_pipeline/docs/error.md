### Error type

`SubmitError::Empty(field_name)` is the carrier's error type. The
validation-failure test pattern-matches on the `Empty` variant
and the field name to confirm the bag-finalize step rejected the
right field.
