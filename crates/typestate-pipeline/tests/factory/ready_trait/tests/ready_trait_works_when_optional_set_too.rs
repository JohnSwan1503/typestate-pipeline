use super::{ProfileFactory, generic_finalize};

pub fn main() {
    let bag = ProfileFactory::new()
        .name("bob".to_owned())
        .handle("@bob".to_owned())
        .with_age(42);
    let profile = generic_finalize(bag);
    assert_eq!(profile.age, 42);
}
