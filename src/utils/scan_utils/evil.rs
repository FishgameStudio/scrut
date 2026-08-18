//! Scan malicious contents.

use regex::Regex;

use super::utils::{Rule, diagnostic_by_regex};
use crate::utils::scan::ItemType as it;

use once_cell::sync::Lazy;

////// Regex static matching //////
pub static EVIL_RULES: Lazy<Rule> = Lazy::new(|| {
    Rule::new(
        "Evil behaviors",
        it::Evil,
        vec![
            Regex::new(r"\bmem::transmute\b").unwrap(),
            Regex::new(r"\b(TcpStream|UdpSocket)::connect\b").unwrap(),
            Regex::new(r"\bptr::(write|copy|copy_nonoverlapping)\b").unwrap(),
            Regex::new(r"\brm -rf\b").unwrap(),
            Regex::new(r"\bdel /f /s\b").unwrap(),
            Regex::new(r"\bchmod 777\b").unwrap(),
            Regex::new(r"remove_dir_all\(").unwrap(),
        ],
    )
});

pub fn scan_evil(s: &String, filename: &String) -> i32 {
    let mut total_error: i32 = 0;
    total_error += diagnostic_by_regex(&*EVIL_RULES, s, filename);
    total_error
}
