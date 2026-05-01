use super::JobFactory;

pub fn main() {
    // Internal fields are read directly out of plain-`T` storage by
    // finalize — no MaybeUninit unwrap, no flag check.
    let job = JobFactory::new("solana".to_owned())
        .parallelism(1)
        .finalize();
    assert_eq!(job.namespace, "solana");
}
