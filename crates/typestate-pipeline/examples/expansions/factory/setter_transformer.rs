use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required, setter = trim_name)]
    name: String,
}

fn trim_name(value: String) -> String {
    value.trim().to_owned()
}

fn main() {
    let user = UserFactory::new()
        .name("   Alice   ".to_owned())
        .finalize();
    assert_eq!(user.name, "Alice");
}
