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
    let cfg = ConfigurableFactory::new()
        .name("svc-b".to_owned())
        .with_parallelism(16)
        .with_url("https://override.example".to_owned())
        .finalize();
    assert_eq!(cfg.parallelism, 16);
    assert_eq!(cfg.url, "https://override.example");
}
