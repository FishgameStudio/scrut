//! A simple logging framework for scrut.

use std::sync::atomic::{AtomicBool, Ordering};

use chrono::Local;

fn now_str() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

static VERBOSE_LOGGING: AtomicBool = AtomicBool::new(false);

/// Enable verbose logging.
pub fn enable_verbose() {
    VERBOSE_LOGGING.store(true, Ordering::Relaxed);
}

/// Disable verbose logging.
pub fn disable_verbose() {
    VERBOSE_LOGGING.store(false, Ordering::Relaxed);
}

/// Verbose logging
macro_rules! verbose {
    ($($arg:tt)*) => {
        if VERBOSE_LOGGING.load(Ordering::Relaxed) {
            print!("[{}] ", now_str());
            println!($($arg)*);
        }
    };
}

pub(crate) use verbose;
