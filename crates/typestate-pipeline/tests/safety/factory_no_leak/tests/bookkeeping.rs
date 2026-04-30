use std::sync::{
    Mutex, MutexGuard,
    atomic::{AtomicUsize, Ordering},
};

pub static ALIVE: AtomicUsize = AtomicUsize::new(0);
pub static LOCK: Mutex<()> = Mutex::new(());

#[derive(Debug)]
pub struct Counted(pub &'static str);

impl Counted {
    pub fn new(label: &'static str) -> Self {
        ALIVE.fetch_add(1, Ordering::SeqCst);
        Counted(label)
    }
}

impl Drop for Counted {
    fn drop(&mut self) {
        ALIVE.fetch_sub(1, Ordering::SeqCst);
    }
}

pub fn alive() -> usize {
    ALIVE.load(Ordering::SeqCst)
}

/// Serialize a test against its peers in this file. Recovers from poisoning
/// so a previous failing test doesn't cascade — each test must report its
/// own leak independently.
pub fn serialize() -> MutexGuard<'static, ()> {
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}
