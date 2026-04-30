use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct LoudUser {
    #[field(required, name = shout_name)]
    name: String,
}

pub fn main() {
    let u = LoudUserFactory::new()
        .shout_name("ALICE".to_owned())
        .finalize();
    assert_eq!(u.name, "ALICE");
}
