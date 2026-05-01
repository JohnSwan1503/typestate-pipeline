use std::sync::{Arc, Mutex};

use super::{Hub, drafted};

pub async fn main() {
    // The InFlight `inspect` returns InFlight, so the chain should
    // continue into `deploy()` — itself an async-deferred transition —
    // and fold everything into one terminal `.await?`.
    let hub = Hub::default();
    let pipeline = drafted(&hub, "delta");

    let log = Arc::new(Mutex::new(Vec::<String>::new()));
    let log_clone = Arc::clone(&log);

    let deployed = pipeline
        .tag()
        .inspect(move |c| {
            log_clone
                .lock()
                .unwrap()
                .push(format!("tag={}", c.state().tag));
        })
        .deploy()
        .await
        .unwrap();

    let log = log.lock().unwrap();
    assert_eq!(*log, vec!["tag=1"]);
    let state = deployed.into_state();
    assert_eq!(state.name, "delta");
    assert_eq!(state.tag, 1);
    assert_eq!(state.job_id, 2);
}
