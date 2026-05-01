use super::{Author, Hub, Tagged, drafted};
use typestate_pipeline::Resolved;

pub async fn main() {
    // The combinator returns `Self` unchanged, so a Resolved-mode
    // inspect inside a chain doesn't disrupt the downstream type. We
    // witness this by chaining `.tag().await?` after the inspect: the
    // type system has to accept `inspect(...)` as still-Drafted for
    // `tag` to be callable on it.
    let hub = Hub::default();
    let pipeline = drafted(&hub, "beta");

    let tagged = pipeline
        .inspect(|c| {
            assert_eq!(c.state().name, "beta");
        })
        .tag()
        .await
        .unwrap();
    let resolved_tagged: Author<Tagged, Resolved> = tagged;
    assert_eq!(resolved_tagged.state().name, "beta");
    assert_eq!(resolved_tagged.state().tag, 1);
}
