use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
struct NormalizedUser {
    #[field(required, setter = trim_name)]
    name: String,
}

fn trim_name(value: String) -> String {
    value.trim().to_owned()
}

pub fn main() {
    let u = NormalizedUserFactory::new()
        .name("   Bob   ".to_owned())
        .finalize();
    assert_eq!(u.name, "Bob");
}
