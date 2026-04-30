use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(name = ManifestBuilder)]
#[allow(dead_code)]
struct Manifest {
    #[field(required, name = shout_title)]
    title: String,
}

fn main() {
    let bag = ManifestBuilder::new().shout_title("DATASET-X".to_owned());
    assert_eq!(bag.title(), "DATASET-X"); // getter name unchanged
    let m = bag.finalize();
    assert_eq!(m.title, "DATASET-X");
}
