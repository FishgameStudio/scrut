//! A file to store version centrally.

pub const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("BUILD_TIME"),
    ") [",
    env!("TARGET_ARCH"),
    "] on ",
    env!("TARGET_OS")
);
