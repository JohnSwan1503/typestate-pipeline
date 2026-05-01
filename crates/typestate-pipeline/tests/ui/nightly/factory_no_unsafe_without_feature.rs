// `#[factory(no_unsafe)]` is rejected at expansion time when the
// `no_unsafe` Cargo feature on the `typestate-pipeline` façade is off.
// Pin the wording so a future macro refactor can't silently degrade it
// into a generic "unknown attribute" message.

use typestate_pipeline::TypestateFactory;

#[derive(TypestateFactory)]
#[factory(no_unsafe)]
struct User {
    #[field(required)]
    name: String,
}

fn main() {}
