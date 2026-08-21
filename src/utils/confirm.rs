//! Store flag status of parameter `-c` `--confirm`.

use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) static CONFIRM_FLAG: AtomicBool = AtomicBool::new(false);

#[inline(always)]
#[allow(dead_code)]
fn get_confirm_flag() -> bool {
    CONFIRM_FLAG.load(Ordering::Relaxed)
}
#[inline(always)]
#[allow(dead_code)]
fn set_confirm_flag(option: bool) {
    CONFIRM_FLAG.store(option, Ordering::Relaxed);
}
