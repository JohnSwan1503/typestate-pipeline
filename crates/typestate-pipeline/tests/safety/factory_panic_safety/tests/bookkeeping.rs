use std::sync::{
    Mutex, MutexGuard,
    atomic::{AtomicUsize, Ordering},
};

pub static COUNTED_ALIVE: AtomicUsize = AtomicUsize::new(0);
pub static PANICKY_FUSE: AtomicUsize = AtomicUsize::new(0);
pub static LOCK: Mutex<()> = Mutex::new(());

pub fn alive() -> usize {
    COUNTED_ALIVE.load(Ordering::SeqCst)
}

/// Reset both counters and arm the one-shot panic. Hold the returned
/// guard for the rest of the test to keep it isolated from peers.
pub fn setup() -> MutexGuard<'static, ()> {
    let g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    COUNTED_ALIVE.store(0, Ordering::SeqCst);
    PANICKY_FUSE.store(1, Ordering::SeqCst);
    g
}

#[derive(Debug)]
pub struct Counted(pub &'static str);

impl Counted {
    pub fn new(label: &'static str) -> Self {
        COUNTED_ALIVE.fetch_add(1, Ordering::SeqCst);
        Counted(label)
    }
}

impl Drop for Counted {
    fn drop(&mut self) {
        COUNTED_ALIVE.fetch_sub(1, Ordering::SeqCst);
    }
}
