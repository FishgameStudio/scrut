//! A file to store version centrally.

pub const VERSION: &str = concat!(
    "{} ({}) [{}] on {}",
    env!("CARGO_PKG_VERSION"),
    env!("BUILD_TIME"),
    env!("TARGET_ARCH"),
    env!("TARGET_OS")
);
