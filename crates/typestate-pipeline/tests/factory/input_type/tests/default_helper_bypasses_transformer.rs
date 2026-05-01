use super::ProfileFactory;

pub fn main() {
    // The `default = None` expression is `Option<String>`, NOT `String`.
    // The default helper must inline a direct field write rather than
    // routing through the setter (which would require `String`). This
    // exercises the bypass path in `gen_default_helper`.
    let bag = ProfileFactory::new()
        .name("bob".to_owned())
        .worker_default();
    let profile = bag.finalize();

    assert_eq!(profile.worker, None);
}
