use typestate_pipeline::TypestateFactory;

#[derive(Debug)]
struct Err1;
impl std::fmt::Display for Err1 {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl std::error::Error for Err1 {}

#[derive(TypestateFactory)]
#[factory(error = Err1)]
struct Bag {
    #[field(required, fallible)]
    name: String,
}

fn main() {}
