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
    // Half the optionals set, half defaulted.
    let cfg = ConfigurableFactory::new()
        .name("svc-c".to_owned())
        .with_parallelism(4) // explicit
        // url left at default
        .finalize();
    assert_eq!(cfg.parallelism, 4);
    assert_eq!(cfg.url, "https://default.example");
}
