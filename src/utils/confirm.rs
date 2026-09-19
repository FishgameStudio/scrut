//! Store flag status of parameter `-c` `--confirm`.

use std::io;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::utils::logging::{ret, verbose, warning};

use owo_colors::OwoColorize;

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
    ret!(CONFIRM_FLAG.load(Ordering::Relaxed))
}
/// Set the status of confirming flag.
#[inline(always)]
pub fn set_confirm_flag(option: bool) {
    ret!(CONFIRM_FLAG.store(option, Ordering::Relaxed));
}
/// Confirm action. Exit if entered `n`, continue if entered `y`.
#[inline]
pub fn confirm(prompt: &str, default_option: ConfirmDefaultOption) {
    if !get_confirm_flag() {
        return;
    }
    verbose!(
        "Confirming choice, with prompt '{prompt}', with default_option '{default_option:?}' ..."
    );
    let msg = format!(
        "{} {prompt} {}: ",
        "Confirm:".yellow().bold(),
        match default_option {
            ConfirmDefaultOption::Yes => "(Y/n)",
            ConfirmDefaultOption::No => "(y/N)",
            ConfirmDefaultOption::None => "(y/n)",
        }
    );
    loop {
        let mut buf = String::new();
        eprint!("{msg}");
        // Read choice from stdin.
        if let Err(e) = io::stdin().read_line(&mut buf) {
            warning!("Failed to read stdin: {e}, retrying");
            continue;
        }
        verbose!("Matching buf '{buf}' ...");
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
