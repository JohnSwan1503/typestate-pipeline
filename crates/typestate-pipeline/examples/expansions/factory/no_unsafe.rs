use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
#[allow(dead_code)]
struct User {
    #[field(required)]
    name: String,
    #[field(default = 18)]
    age: u32,
}

fn main() {
    let user = UserFactory::new().name("Alice".to_owned()).finalize();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.age, 18);

    let user = UserFactory::new()
        .name("Bob".to_owned())
        .with_age(42)
        .finalize();
    assert_eq!(user.age, 42);
}
