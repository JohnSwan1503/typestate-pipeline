use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct Server {
    pub next_id: AtomicU64,
}

impl Server {
    pub fn allocate_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::SeqCst) + 1
    }
}
