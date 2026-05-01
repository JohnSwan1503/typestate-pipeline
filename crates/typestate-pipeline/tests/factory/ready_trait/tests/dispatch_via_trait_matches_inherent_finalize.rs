use super::{ProfileFactory, ProfileFactoryReady};

pub fn main() {
    // The trait method must produce the exact same value as the
    // inherent — it just delegates. This catches regressions where
    // the trait impl body diverges from the inherent's body (e.g.
    // forgetting to apply optional-with-default branching).
    let trait_path = ProfileFactory::new()
        .name("c".to_owned())
        .handle("@c".to_owned())
        .with_age(7);
    let inherent_path = ProfileFactory::new()
        .name("c".to_owned())
        .handle("@c".to_owned())
        .with_age(7);

    let via_trait = ProfileFactoryReady::finalize(trait_path);
    let via_inherent = inherent_path.finalize();

    assert_eq!(via_trait.name, via_inherent.name);
    assert_eq!(via_trait.handle, via_inherent.handle);
    assert_eq!(via_trait.age, via_inherent.age);
}
