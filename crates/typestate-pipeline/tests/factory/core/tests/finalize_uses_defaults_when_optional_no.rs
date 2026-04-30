use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct Configurable {
    #[field(required)]
    name: String,
    #[field(default = 8)]
    parallelism: u16,
    #[field(default = "https://default.example".to_owned())]
    url: String,
}

pub fn main() {
    // Don't set parallelism or url — finalize must still succeed and
    // use the declared defaults.
    let cfg = ConfigurableFactory::new()
        .name("svc-a".to_owned())
        .finalize();
    assert_eq!(cfg.name, "svc-a");
    assert_eq!(cfg.parallelism, 8);
    assert_eq!(cfg.url, "https://default.example");
}
