use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct UserBuilder {
    #[field(required)]
    name: String,
    #[field(required)]
    email: String,
    #[field(default = 18)]
    age: u32,
}

pub fn main() {
    // Setter ordering is irrelevant — the typestate transitions
    // independently per field.
    let bag = UserBuilderFactory::new()
        .with_age(42)
        .email("bob@example.com".to_owned())
        .name("Bob".to_owned());
    let user = bag.finalize();

    assert_eq!(user.name, "Bob");
    assert_eq!(user.email, "bob@example.com");
    assert_eq!(user.age, 42);
}
