use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required)]
    name: String,
    #[field(default = 18)]
    age: u32,
}

/// Generic over any finalize-callable shape — a stand-in for a downstream
/// API that wants to accept "any built bag" without spelling out the flag
/// tuple.
fn finalize_anything<B: UserFactoryReady>(bag: B) -> User {
    bag.finalize()
}

fn main() {
    // Required `name` set, optional `age` left to default — Ready.
    let bag = UserFactory::new().name("Alice".to_owned());
    let user = finalize_anything(bag);
    assert_eq!(user.age, 18);

    // Same trait dispatch works when the optional was set explicitly.
    let bag = UserFactory::new().name("Bob".to_owned()).with_age(42);
    let user = finalize_anything(bag);
    assert_eq!(user.age, 42);
}
