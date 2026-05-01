use typestate_pipeline::No;

use super::JobFactory;

pub fn main() {
    // The bag's type signature is `JobFactory<ParallelismFlag, VerifyFlag>` —
    // only two generics, NOT three. If the internal field had a flag,
    // the type below would need `JobFactory<Yes, No, No>` instead. The
    // fact that this annotation typechecks is the witness.
    let bag: JobFactory<No, No> = JobFactory::new("eth".to_owned());
    let _ = bag;
}
