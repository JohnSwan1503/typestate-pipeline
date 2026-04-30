#[path = "shared.rs"]
mod shared;

use shared::JobFactory;

pub fn main() {
    // Bag in the all-No-flags state still has `namespace()` callable —
    // the internal field has no `Yes`-required bound.
    let bag = JobFactory::new("op".to_owned());
    assert_eq!(bag.namespace(), "op");
}
