use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required, removable)]
    name: String,
    #[field(required)]
    age: u32,
}

fn main() {
    let bag = UserFactory::new().name("draft".to_owned()).age(30);

    // Drop just the name; flag goes Yes -> No.
    let bag = bag.drop_name();

    // Set it again with the final value; flag goes No -> Yes.
    let user = bag.name("Alice".to_owned()).finalize();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.age, 30);
}
