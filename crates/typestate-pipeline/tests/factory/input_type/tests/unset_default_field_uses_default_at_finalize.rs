#[path = "shared.rs"]
mod shared;

use shared::ProfileFactory;

pub fn main() {
    // Same end-state as `worker_default()` but reached via the bag's
    // optional-field finalize bound (flag stays `No`, the default
    // expr is evaluated by `finalize`).
    let profile = ProfileFactory::new().name("carol".to_owned()).finalize();
    assert_eq!(profile.worker, None);
}
