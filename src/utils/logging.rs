//! A simple logging framework for scrut.

use std::fs::{File, OpenOptions};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

use chrono::Local;

use dirs::home_dir;

#[inline(always)]
#[allow(unused)]
pub fn now_str() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[allow(dead_code)]
pub static VERBOSE_LOGGING: AtomicBool = AtomicBool::new(false);
/// Global handle of log file.
#[allow(dead_code)]
pub static LOG_FILE: Mutex<Option<File>> = Mutex::new(None);

/// Enable verbose logging, open log file.
/// # Examples
/// ```
/// crate::utils::logging::enable_verbose().expect("cannot open verbose log file");
/// ```
pub fn enable_verbose() -> std::io::Result<()> {
    VERBOSE_LOGGING.store(true, Ordering::Relaxed);

    let mut guard = LOG_FILE.lock().unwrap();
    if guard.is_none() {
        let log_path = format!("{}/scrut.log", home_dir().unwrap().to_str().unwrap());
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;
        *guard = Some(file);
    }
    Ok(())
}

/// Disable verbose logging, close log file.
#[allow(dead_code)]
pub fn disable_verbose() {
    VERBOSE_LOGGING.store(false, Ordering::Relaxed);
    // Free file.
    *LOG_FILE.lock().unwrap() = None;
}

/// Returns the current function name.
macro_rules! func_name {
    () => {{
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let closure = || ();
        type_name_of(closure)
    }};
}
pub(crate) use func_name;

/// Verbose logging
macro_rules! verbose {
    ($($arg:tt)*) => {
        if crate::utils::logging::VERBOSE_LOGGING.load(std::sync::atomic::Ordering::Relaxed) {
            use std::io::Write;
            let args = format_args!($($arg)*);
            let timestamp = crate::utils::logging::now_str();
            let func = crate::utils::logging::func_name!();
            let line = format!("[{} in {}] {}\n", timestamp, func, args);

            // Console output
            print!("{}", line);

            // Write into the log file.
            if let Ok(mut guard) = crate::utils::logging::LOG_FILE.lock() {
                if let Some(ref mut f) = *guard {
                    let _ = f.write_all(line.as_bytes());
                    let _ = f.flush(); // write immediately
                }
            }
        }
    };
}
pub(crate) use verbose;
