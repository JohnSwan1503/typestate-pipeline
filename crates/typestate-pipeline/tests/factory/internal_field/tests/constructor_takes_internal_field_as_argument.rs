#[path = "shared.rs"]
mod shared;

use shared::JobFactory;

pub fn main() {
    // No `.namespace(…)` chain — the field is supplied to `new(…)`.
    let bag = JobFactory::new("eth".to_owned())
        .parallelism(4)
        .with_verify(true);
    let job = bag.finalize();
    assert_eq!(job.namespace, "eth");
    assert_eq!(job.parallelism, 4);
    assert!(job.verify);
}
