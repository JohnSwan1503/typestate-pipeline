use std::sync::{
    Mutex, MutexGuard,
    atomic::{AtomicUsize, Ordering},
};

// ---------------------------------------------------------------------------
// Drop bookkeeping
// ---------------------------------------------------------------------------

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

/// Serialize a leak-tracking test against its peers in this binary so
/// the shared `ALIVE` static can't be raced by another running test.
/// Recovers from poisoning so a previous failure reports independently
/// rather than cascading.
pub fn serialize() -> MutexGuard<'static, ()> {
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}
