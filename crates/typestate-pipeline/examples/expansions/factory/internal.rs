use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[allow(dead_code)]
struct Job {
    #[field(required, internal)]
    namespace: String,
    #[field(required)]
    parallelism: u16,
    #[field(default = false)]
    verify: bool,
}

fn main() {
    // No `.namespace(...)` chain — the field is positional on `new(...)`.
    let bag = JobFactory::new("eth".to_owned())
        .parallelism(4)
        .with_verify(true);

    // The namespace getter is callable on any bag shape — even before
    // `parallelism` is set:
    let fresh = JobFactory::new("op".to_owned());
    assert_eq!(fresh.namespace(), "op");

    let job = bag.finalize();
    assert_eq!(job.namespace, "eth");
    assert_eq!(job.parallelism, 4);
    assert!(job.verify);
}
