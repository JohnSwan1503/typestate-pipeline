use typestate_pipeline::TypestateFactory;

// `default_helper = …` only makes sense alongside `default = …` since the
// helper applies the declared default. Using it on a required field with
// no default is a hard error so the misuse is caught at the derive site
// instead of mysteriously producing no helper method.
#[derive(TypestateFactory)]
struct Bag {
    #[field(required, default_helper = my_helper)]
    name: String,
}

fn main() {}
