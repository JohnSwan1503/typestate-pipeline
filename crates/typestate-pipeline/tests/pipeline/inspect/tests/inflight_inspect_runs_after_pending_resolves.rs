use std::sync::{Arc, Mutex};

use typestate_pipeline::Resolved;

use super::{Author, Hub, Tagged, drafted};

pub async fn main() {
    // Set up a pipeline that's about to enter the chain. `tag()` is
    // async-deferred, so the result is `Author<Tagged, InFlight>`.
    // Calling `.inspect(...)` on InFlight defers the closure until the
    // future is awaited.
    let hub = Hub::default();
    let pipeline = drafted(&hub, "gamma");

    // Use a Mutex to record observations from the closure (it runs
    // inside an `async move` and needs `Send`).
    let log = Arc::new(Mutex::new(Vec::<String>::new()));
    let log_for_inspect = Arc::clone(&log);

    // Before-await: the closure must NOT have run yet.
    let chain = pipeline.tag().inspect(move |c| {
        log_for_inspect.lock().unwrap().push(format!(
            "inspected name={} tag={}",
            c.state().name,
            c.state().tag
        ));
    });
    assert!(
        log.lock().unwrap().is_empty(),
        "deferred inspect should NOT run until the chain is awaited"
    );

    // After-await: the closure has run, observed the resolved Tagged
    // state, and the chain has resolved to a Resolved carrier whose
    // state matches what the closure saw.
    let resolved: Author<Tagged, Resolved> = chain.await.unwrap();
    assert_eq!(*log.lock().unwrap(), vec!["inspected name=gamma tag=1"]);
    assert_eq!(resolved.state().name, "gamma");
    assert_eq!(resolved.state().tag, 1);
}
