//! `<BagName>Ready` must not be implemented when a required field's flag
//! is `No`. Calling `finalize()` on a partial bag must fail to
//! compile.

use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
struct Profile {
    #[field(required)]
    name: String,
    #[field(required)]
    handle: String,
}

fn main() {
    let bag = ProfileFactory::new().name("alice".to_owned());
    // ERROR: `handle`'s flag is `No`, so `ProfileReady` isn't implemented.
    let _ = bag.finalize();
}
