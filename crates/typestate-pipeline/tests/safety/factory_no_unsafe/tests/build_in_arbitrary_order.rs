use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
struct UserBuilder {
    #[field(required)]
    name: String,
    #[field(required)]
    email: String,
    #[field(default = 18)]
    age: u32,
}

pub fn main() {
    let user = UserBuilderFactory::new()
        .with_age(42)
        .email("bob@example.com".to_owned())
        .name("Bob".to_owned())
        .finalize();
    assert_eq!(user.name, "Bob");
    assert_eq!(user.age, 42);
}
