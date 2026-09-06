//! Store flag status of parameter `-c` `--confirm`.

use std::io;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::utils::logging::{verbose, warning};

/// Enumeration of default option in confirming input.
#[derive(Debug)]
#[allow(dead_code)]
pub enum ConfirmDefaultOption {
    Yes,
    No,
    None,
}

pub type DefaultOpt = ConfirmDefaultOption;

pub(crate) static CONFIRM_FLAG: AtomicBool = AtomicBool::new(false);

/// Get whether the --confirm argument was given.
#[inline(always)]
pub fn get_confirm_flag() -> bool {
    CONFIRM_FLAG.load(Ordering::Relaxed)
}
/// Set the status of confirming flag.
#[inline(always)]
pub fn set_confirm_flag(option: bool) {
    CONFIRM_FLAG.store(option, Ordering::Relaxed);
}
/// Confirm action. Exit if entered `n`, continue if entered `y`.
#[inline]
pub fn confirm(prompt: &str, default_option: ConfirmDefaultOption) {
    verbose!(
        "Confirming choice, with prompt '{prompt}', with default_option '{default_option:?}' ..."
    );
    println!(
        "{prompt} {}: ",
        match default_option {
            ConfirmDefaultOption::Yes => "(Y/n)",
            ConfirmDefaultOption::No => "(y/N)",
            ConfirmDefaultOption::None => "(y/n)",
        }
    );
    let mut buf = String::new();
    loop {
        // Read choice from stdin.
        if let Err(e) = io::stdin().read_line(&mut buf) {
            warning!("Failed to read stdin: {e}, retrying");
            continue;
        }
        match buf.to_lowercase().trim() {
            "" => match default_option {
                ConfirmDefaultOption::None => continue,
                ConfirmDefaultOption::Yes => break,
                ConfirmDefaultOption::No => exit(1),
            },
            "y" => break,
            "n" => exit(1),
            _ => continue,
        }
    }
}
/// Confirm action. Exit if entered `n`.
#[inline]
#[allow(dead_code)]
pub fn confirm_noexit(prompt: &str, default_option: ConfirmDefaultOption) -> bool {
    verbose!(
        "Confirming choice, with prompt '{prompt}', with default_option '{default_option:?}' ..."
    );
    println!(
        "{prompt} {}: ",
        match default_option {
            ConfirmDefaultOption::Yes => "(Y/n)",
            ConfirmDefaultOption::No => "(y/N)",
            ConfirmDefaultOption::None => "(y/n)",
        }
    );
    let mut buf = String::new();
    loop {
        // Read choice from stdin.
        if let Err(e) = io::stdin().read_line(&mut buf) {
            warning!("Failed to read stdin: {e}, retrying");
            continue;
        }
        match buf.to_lowercase().trim() {
            "" => match default_option {
                ConfirmDefaultOption::None => continue,
                ConfirmDefaultOption::Yes => return true,
                ConfirmDefaultOption::No => return false,
            },
            "y" => return true,
            "n" => exit(1),
            _ => return false,
        }
    }
}
