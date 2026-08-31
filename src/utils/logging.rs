//! A simple logging framework for scrut.

use std::fs::{File, OpenOptions};
use std::io;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

use chrono::Local;
use dirs::home_dir;

////// Macro functions for color escaping //////

/// Generates a string wrapped with ANSI red escape sequences.
#[allow(unused_macros)]
macro_rules! red {
    ($s:expr) => {
        format!("\x1b[31m{}\x1b[0m", $s)
    };
}
/// Generates a string wrapped with ANSI yellow escape sequences.
#[allow(unused_macros)]
macro_rules! yellow {
    ($s:expr) => {
        format!("\x1b[33m{}\x1b[0m", $s)
    };
}
/// Generates a string wrapped with ANSI magenta‑purple escape sequences.
macro_rules! purple {
    ($s:expr) => {
        format!("\x1b[35m{}\x1b[0m", $s)
    };
}
#[allow(unused_imports)]
pub(crate) use {purple, red, yellow};

#[inline(always)]
pub fn now_str() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

////// Logging //////

/// Controls whether verbose messages print to console.
/// File logging is always enabled regardless of this flag.
pub static VERBOSE_LOGGING: AtomicBool = AtomicBool::new(false);

/// Global handle of log file, always opened on startup.
pub static LOG_FILE: Mutex<Option<File>> = Mutex::new(None);

/// Max size of log file.
pub const MAX_LOG_BYTES: u64 = 10 * 1024 * 1024;

/// Initialize log file, call once at program start(main).
/// Always open log file, independent of verbose flag.
/// # Examples
/// ```
/// use std::error::Error;
/// fn main() -> Result<(), Box<dyn Error>> {
///     init_log_file(None);           // Init with default size.
///     init_log_file(Some(5 * 1024 * 1024)) // Init with specified size.
/// }
/// ```
pub fn init_log_file(max_log_bytes: Option<u64>) -> io::Result<()> {
    let home = home_dir().ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "failed to get home directory",
    ))?;
    let home_str = home.to_str().ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        "home path invalid utf‑8",
    ))?;
    let log_path = format!("{}/scrut.log", home_str);
    // Clear log file if size is greater than 10MB.
    let max_log_bytes: u64 = max_log_bytes.unwrap_or(MAX_LOG_BYTES);
    if std::path::Path::new(&log_path).exists() {
        let meta = std::fs::metadata(&log_path)?;
        if meta.len() > max_log_bytes {
            // Open and truncate, clear old logs.
            let _ = File::create(&log_path)?;
        }
    }

    let mut guard = LOG_FILE.lock().unwrap();
    if guard.is_none() {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;
        *guard = Some(file);
    }
    Ok(())
}

/// Enable console verbose output, file logging is already active.
pub fn enable_verbose() {
    VERBOSE_LOGGING.store(true, Ordering::Relaxed);
}

/// Disable console verbose output, file logging still works.
#[allow(dead_code)]
pub fn disable_verbose() {
    VERBOSE_LOGGING.store(false, Ordering::Relaxed);
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
        {
            use std::io::Write;
            use std::sync::atomic::Ordering;
            use std::thread;
            use crate::utils::logging as this;
            let args = format_args!($($arg)*);
            let timestamp = this::now_str();
            let func = this::func_name!();
            let curr_thread = thread::current();
            let thread_name = curr_thread.name().unwrap_or("main");
            let line = format!("[{} thread '{}' in '{}'] {}\n", thread_name, timestamp, func, args);

            // Print to console ONLY when verbose flag is on
            if this::VERBOSE_LOGGING.load(Ordering::Relaxed) {
                print!("{} {}", this::purple!("Verbose:"), line);
            }

            // ALWAYS write to log file (if file handle exists)
            if let Ok(mut guard) = this::LOG_FILE.lock() {
                if let Some(ref mut f) = *guard {
                    let _ = f.write_all(format!("[v] {}", line).as_bytes());
                    let _ = f.flush();
                }
            }
        }
    };
}
pub(crate) use verbose;

/// Print warning message and write into the log file.
macro_rules! warning {
    ($($arg:tt)*) => {
        {
            use std::io::Write;
            use crate::utils::logging as this;
            let args = format_args!($($arg)*);
            let timestamp = this::now_str();
            let func = this::func_name!();
            let line = format!("{}\n", args);

            eprint!("{} {}", this::yellow!("Warning:"), line);

            // Write to the log file.
            if let Ok(mut guard) = this::LOG_FILE.lock() {
                if let Some(ref mut f) = *guard {
                    let line = format!("[w] [{} in {}] {}\n", timestamp, func, args);
                    let _ = f.write_all(line.as_bytes());
                    let _ = f.flush();
                }
            }
        }
    };
}
pub(crate) use warning;

/// Print fatal message and write into the log file.
/// This macro will **automatically panic** if matched arm #0.
macro_rules! fatal {
    ($($arg:tt)*) => {
        {
            use std::io::Write;
            use crate::utils::logging as this;
            use std::thread;
            let args = format_args!($($arg)*);
            let timestamp = this::now_str();
            let func = this::func_name!();
            let curr_thread = thread::current();
            let thread_name = curr_thread.name().unwrap_or("main");
            let line = format!("{}\n", args);

            eprint!("{} {}", this::red!("Fatal:"), line);

            // Write to the log file.
            if let Ok(mut guard) = this::LOG_FILE.lock() {
                if let Some(ref mut f) = *guard {
                    let line = format!("[f] [{} thread '{}' panicked in '{}'] {}\n", thread_name, timestamp, func, args);
                    let _ = f.write_all(line.as_bytes());
                    let _ = f.flush();
                }
            }

            // Panic
            panic!("{}", args);
        }
    };
    (no_panic; $($arg:tt)*) => {
        {
            use std::io::Write;
            use std::thread;
            use crate::utils::logging as this;
            let args = format_args!($($arg)*);
            let timestamp = this::now_str();
            let func = this::func_name!();
            let curr_thread = thread::current();
            let thread_name = curr_thread.name().unwrap_or("main");
            let line = format!("[{} thread '{}' panicked in '{}'] {}\n", thread_name, timestamp, func, args);

            eprint!("{} {}", this::red!("Fatal:"), line);

            // Write to the log file.
            if let Ok(mut guard) = this::LOG_FILE.lock() {
                if let Some(ref mut f) = *guard {
                    let _ = f.write_all(format!("[f] {}", line).as_bytes());
                    let _ = f.flush();
                }
            }
        }
    };
}
pub(crate) use fatal;
