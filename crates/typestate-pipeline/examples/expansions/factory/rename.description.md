### Renaming: when the defaults clash

The previous five sections covered every flag transition the
factory understands. The next batch covers the *setter body* —
what runs inside the method when a value is being stored. Before
diving in, one cosmetic detour: the names of the generated
items.

`#[factory(name = MyType)]` overrides the bag type's name (the
default is `<Original>Factory`). `#[field(name = my_setter)]`
overrides a setter's method name. Both knobs exist for cases
where the default name collides with surrounding code, or where
the call site reads better with a different verb.

Two things the field-rename does *not* touch: the default-helper
name (still `<field>_default`) and the getter name (still the
field name). To rename the helper, use
`#[field(default_helper = my_helper)]`. The getter is
intentionally locked to the field name — the getter is reading
the *field*, not the *setter*, and the rename is a setter
concern.
