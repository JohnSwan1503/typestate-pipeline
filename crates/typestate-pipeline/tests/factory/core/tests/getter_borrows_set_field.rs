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
    let bag = UserBuilderFactory::new()
        .name("Dave".to_owned())
        .email("dave@example.com".to_owned());

    // Both required-flag fields are now Yes; the getters are available.
    assert_eq!(bag.name(), "Dave");
    assert_eq!(bag.email(), "dave@example.com");

    // Default the optional and finalize.
    let user = bag.age_default().finalize();
    assert_eq!(user.name, "Dave");
}
