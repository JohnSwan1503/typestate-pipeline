#[path = "shared.rs"]
mod shared;

use shared::ProfileFactory;

pub fn main() {
    // The setter accepts a `String`, not an `Option<String>`. The
    // transformer wraps it into the storage shape internally.
    let bag = ProfileFactory::new()
        .name("alice".to_owned())
        .with_worker("worker-1".to_owned());
    let profile = bag.finalize();

    assert_eq!(profile.name, "alice");
    assert_eq!(profile.worker, Some("worker-1".to_owned()));
}
