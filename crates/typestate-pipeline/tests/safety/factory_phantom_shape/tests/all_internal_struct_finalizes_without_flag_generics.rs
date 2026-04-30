use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct AllInternal {
    #[field(internal)]
    namespace: &'static str,
    #[field(internal)]
    version: u32,
}

pub fn main() {
    let s = AllInternalFactory::new("svc", 7).finalize();
    assert_eq!(s.namespace, "svc");
    assert_eq!(s.version, 7);
}
