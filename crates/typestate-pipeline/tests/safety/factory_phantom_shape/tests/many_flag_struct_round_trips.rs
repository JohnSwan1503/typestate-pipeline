use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct ThreeFlags {
    #[field(required)]
    a: u32,
    #[field(required)]
    b: u32,
    #[field(required)]
    c: u32,
}

pub fn main() {
    let s = ThreeFlagsFactory::new().a(1).b(2).c(3).finalize();
    assert_eq!((s.a, s.b, s.c), (1, 2, 3));
}
