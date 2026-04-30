use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
struct WithInternal {
    #[field(internal)]
    namespace: String,
    #[field(required)]
    name: String,
}

pub fn main() {
    let bag = WithInternalFactory::new("ns".to_owned());
    // Internal getter is callable on the empty bag.
    assert_eq!(bag.namespace(), "ns");

    let user = bag.name("svc".to_owned()).finalize();
    assert_eq!(user.namespace, "ns");
    assert_eq!(user.name, "svc");
}
