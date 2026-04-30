use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct CollidingFieldNames {
    /// Previously collided with `_markers: PhantomData<…>` on the bag.
    #[field(required)]
    _markers: u32,
    /// Same family — would have shadowed the macro-internal `this`
    /// binding if a `default = …` expression referenced `this.this`.
    #[field(required)]
    this: u32,
    /// Same family — would have shadowed `__field_value`.
    #[field(required)]
    __field_value: u32,
    /// And the override / remove temps.
    #[field(required, overridable, removable)]
    __old_field: u32,
    #[field(required, overridable, removable)]
    __new_bag: u32,
}

pub fn main() {
    let s = CollidingFieldNamesFactory::new()
        ._markers(1)
        .this(2)
        .__field_value(3)
        .__old_field(4)
        .__new_bag(5)
        .finalize();
    assert_eq!(s._markers, 1);
    assert_eq!(s.this, 2);
    assert_eq!(s.__field_value, 3);
    assert_eq!(s.__old_field, 4);
    assert_eq!(s.__new_bag, 5);
}
