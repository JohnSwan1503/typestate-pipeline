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
    let bag = UserBuilderFactory::new()
        .name("Eve".to_owned())
        .email("eve@example.com".to_owned());
    assert_eq!(bag.name(), "Eve");
    assert_eq!(bag.email(), "eve@example.com");
}
