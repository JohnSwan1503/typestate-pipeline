use super::{ProfileFactory, generic_finalize};

pub fn main() {
    let bag = ProfileFactory::new()
        .name("alice".to_owned())
        .handle("@alice".to_owned());
    // `age` flag stays `No` — bag should still be `Ready` because
    // `age` is optional-with-default.
    let profile = generic_finalize(bag);
    assert_eq!(profile.name, "alice");
    assert_eq!(profile.handle, "@alice");
    assert_eq!(profile.age, 18);
}
