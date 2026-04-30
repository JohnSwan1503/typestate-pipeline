use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required)]
    name: String,
    #[field(optional)]
    nickname: String,
}

fn main() {
    let bag = UserFactory::new()
        .name("Alice".to_owned())
        .with_nickname("Al".to_owned());
    let user = bag.finalize();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.nickname, "Al");
}
