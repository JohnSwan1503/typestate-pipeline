use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(name = MyManifestBuilder)]
struct ManifestData {
    #[field(required)]
    title: String,
}

pub fn main() {
    let m = MyManifestBuilder::new()
        .title("dataset-x".to_owned())
        .finalize();
    assert_eq!(m.title, "dataset-x");
}
