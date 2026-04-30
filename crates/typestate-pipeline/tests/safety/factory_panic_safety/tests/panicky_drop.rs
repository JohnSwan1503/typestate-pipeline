use std::sync::atomic::Ordering;

use super::bookkeeping::PANICKY_FUSE;

/// Panics on its first drop and behaves normally thereafter. The
/// one-shot fuse exists because the `override` / `drop_<field>` tests
/// construct a fresh `PanickyDrop` as the new value; a second panic in
/// `Drop` during unwind would abort the process.
pub struct PanickyDrop;

impl Drop for PanickyDrop {
    fn drop(&mut self) {
        if PANICKY_FUSE
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1))
            .is_ok()
        {
            panic!("PanickyDrop's Drop panicked (one-shot)");
        }
    }
}
