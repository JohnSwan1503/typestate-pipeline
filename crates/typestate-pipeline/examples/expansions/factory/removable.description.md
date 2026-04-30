### Removable: un-setting a field

So far the bag's flags can move only one way: `No → Yes`.
`#[field(removable)]` opens the reverse direction. It emits a
`drop_<field>(self)` method that consumes a `Yes`-flagged bag,
drops the stored value, and returns a bag whose flag is back to
`No`. The setter can then be called again with a new value.

The drop is exact: only the named field is touched; other set
fields keep their values and their `Yes` flags. That's why the
sketch's signature only mentions one flag at a time.

Concretely: imagine a connection field that has to be visibly
torn down before its replacement opens, with the surrounding
phase guarded by branches that may or may not supply a new one.
`drop_<field>` makes the gap show up at the type level — between
the drop and the next setter, the bag carries `No` for that
flag, and any code that tries to use the field has to typecheck
against the unset shape. When the value just needs to be
replaced in place with no observable gap, the next recipe's
`overridable` is shorter.
