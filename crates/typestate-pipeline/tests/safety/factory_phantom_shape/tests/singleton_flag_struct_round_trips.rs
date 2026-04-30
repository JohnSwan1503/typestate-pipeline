use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
pub struct OneFlag {
    #[field(required)]
    pub name: String,
}

pub fn main() {
    let s = OneFlagFactory::new().name("hello".to_owned()).finalize();
    assert_eq!(s.name, "hello");
}
