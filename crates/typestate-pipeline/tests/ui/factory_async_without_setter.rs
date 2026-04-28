use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct Bag {
    #[field(required, async_fn)]
    name: String,
}

fn main() {}
