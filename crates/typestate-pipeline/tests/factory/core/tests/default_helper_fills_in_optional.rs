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
    // `age_default()` is generated because `age` has
    // `#[field(default = 18)]`. It transitions the flag to Yes using
    // the declared default expression.
    let user = UserBuilderFactory::new()
        .name("Carol".to_owned())
        .email("carol@example.com".to_owned())
        .age_default()
        .finalize();

    assert_eq!(user.age, 18);
}
