use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct User {
    #[field(required, overridable)]
    name: String,
}

fn main() {
    // Setter: No -> Yes; override_: Yes -> Yes.
    let user = UserFactory::new()
        .name("draft".to_owned())
        .override_name("Alice".to_owned())
        .finalize();
    assert_eq!(user.name, "Alice");
}
