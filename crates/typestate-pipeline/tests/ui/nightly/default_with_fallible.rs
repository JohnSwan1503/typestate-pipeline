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
    #[field(default = String::new(), setter = trim, fallible)]
    name: String,
}

fn trim(s: String) -> Result<String, Err1> {
    Ok(s)
}

fn main() {}
