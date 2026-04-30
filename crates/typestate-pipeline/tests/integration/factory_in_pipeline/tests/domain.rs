use std::sync::atomic::{AtomicU64, Ordering};

use typestate_pipeline::TypestateFactory;

#[derive(Debug, Default)]
pub struct Server {
    pub next_confirmation_id: AtomicU64,
}

impl Server {
    pub fn confirm(&self, _user: &User) -> u64 {
        self.next_confirmation_id.fetch_add(1, Ordering::SeqCst) + 1
    }
}

#[derive(Debug, Clone, TypestateFactory)]
pub struct User {
    #[field(required)]
    pub name: String,
    #[field(required)]
    pub email: String,
    #[field(default = 18)]
    pub age: u32,
}
