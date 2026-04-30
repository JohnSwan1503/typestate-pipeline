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
        .name("Carol".to_owned())
        .email("carol@example.com".to_owned())
        .age_default()
        .finalize();
    assert_eq!(user.age, 18);
}
