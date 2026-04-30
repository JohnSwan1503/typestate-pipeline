#[path = "shared.rs"]
mod shared;

use std::cell::RefCell;

use typestate_pipeline::Resolved;

use shared::{Author, Drafted, Hub, drafted};

pub async fn main() {
    let hub = Hub::default();
    let pipeline = drafted(&hub, "alpha");

    // Resolved → Resolved: closure must run synchronously and observe
    // the carrier's current state.
    let observed = RefCell::new(None::<String>);
    let _: Author<Drafted, Resolved> = pipeline.inspect(|c| {
        *observed.borrow_mut() = Some(c.state().name.clone());
    });

    assert_eq!(observed.borrow().as_deref(), Some("alpha"));
}
