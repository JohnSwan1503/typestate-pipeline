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
        .name("Alice".to_owned())
        .email("alice@example.com".to_owned())
        .with_age(30);
    let user = bag.finalize();

    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
    assert_eq!(user.age, 30);
}
