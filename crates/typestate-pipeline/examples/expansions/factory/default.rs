use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required)]
    name: String,
    #[field(default = 18)]
    age: u32,
}

fn main() {
    // 1) Skip `age` entirely — the declared default fires at finalize().
    let user = UserFactory::new().name("Alice".to_owned()).finalize();
    assert_eq!(user.age, 18);

    // 2) Set `age` explicitly (note `with_` prefix because age is optional).
    let user = UserFactory::new()
        .name("Bob".to_owned())
        .with_age(30)
        .finalize();
    assert_eq!(user.age, 30);

    // 3) Use the helper to flip the flag without specifying the value.
    let user = UserFactory::new()
        .name("Carol".to_owned())
        .age_default()
        .finalize();
    assert_eq!(user.age, 18);
}
