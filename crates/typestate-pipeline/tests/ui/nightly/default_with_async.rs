use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct Bag {
    #[field(default = 0u32, setter = parse, async_fn)]
    n: u32,
}

async fn parse(n: u32) -> u32 {
    n
}

fn main() {}
