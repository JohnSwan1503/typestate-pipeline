use typestate_pipeline::TypestateFactory;

fn user_helper() -> u32 {
    42
}

#[derive(TypestateFactory)]
struct DefaultUsesUserScope {
    #[field(required)]
    name: String,
    /// The default expression resolves a free function in the user's
    /// scope. Hygiene fix must not block this.
    #[field(default = user_helper())]
    answer: u32,
}

pub fn main() {
    let s = DefaultUsesUserScopeFactory::new()
        .name("Alice".to_owned())
        .finalize();

    assert_eq!(s.answer, 42);
    assert_eq!(s.name, "Alice");
}
