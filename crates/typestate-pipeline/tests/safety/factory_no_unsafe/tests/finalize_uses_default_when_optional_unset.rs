use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
struct UserBuilder {
    #[field(required)]
    name: String,
    #[field(required)]
    email: String,
    #[field(default = 18)]
    age: u32,
}

pub fn main() {
    // Optional-with-default field never set → finalize uses the declared
    // default. In safe mode this dispatches via `Storage::finalize_or` rather
    // than an `IS_SET` runtime branch.
    let user = UserBuilderFactory::new()
        .name("Dave".to_owned())
        .email("dave@example.com".to_owned())
        .finalize();
    assert_eq!(user.age, 18);
}
