use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct User {
    #[field(required)]
    name: String,
    #[field(required)]
    age: u32,
}

fn main() {
    let bag = UserFactory::new().name("Alice".to_owned()).age(30);

    // Getters resolve once the matching flag is `Yes`.
    assert_eq!(bag.name(), "Alice");
    assert_eq!(*bag.age(), 30);

    let user = bag.finalize();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.age, 30);
}
