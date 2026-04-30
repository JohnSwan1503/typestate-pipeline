use typestate_pipeline::TypestateFactory;

#[derive(Debug, TypestateFactory)]
pub struct Profile {
    #[field(required)]
    pub name: String,
    #[field(required)]
    pub handle: String,
    #[field(default = 18)]
    pub age: u32,
}

/// Generic over any `ProfileFactoryReady` bag — a stand-in for a
/// downstream user's `where B: ProfileFactoryReady` bound. Compiling
/// this fn at all is the witness that the macro emitted the trait +
/// impl correctly.
pub fn generic_finalize<B: ProfileFactoryReady>(bag: B) -> Profile {
    bag.finalize()
}
