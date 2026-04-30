use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required)]
    name: String,
    #[field(default = 18)]
    age: u32,
}

/// Function takes the entry-side shape — `UserFactory<No, No>` — without
/// spelling out the tuple. The `UserFactoryEmpty` alias is auto-generated
/// alongside the bag.
fn fresh_user_form() -> UserFactoryEmpty {
    UserFactory::new()
}

fn main() {
    let user = fresh_user_form()
        .name("Alice".to_owned())
        .with_age(30)
        .finalize();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.age, 30);
}
