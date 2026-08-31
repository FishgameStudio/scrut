//! Utilities for this scan module.

use crate::utils::logging::verbose;
use crate::utils::scan::ItemType as it;

use regex::Regex;

/// Regex struct for message printing.
#[derive(Debug, Clone)]
pub struct Rule {
    pub prompt: &'static str,
    pub kind: it,
    pub regex: Vec<Regex>, // owned
}

impl Rule {
    /// Create a new `Rule` object.
    pub fn new(prompt: &'static str, kind: it, regex: Vec<Regex>) -> Self {
        Self {
            prompt,
            kind,
            regex,
        }
    }
}

/// Match position metadata containing 1‑based line number, start and end byte offsets.
#[derive(Debug)]
pub struct MatchPos {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl MatchPos {
    /// Create a new `MatchPos` object.
    pub fn new(line: usize, start: usize, end: usize) -> Self {
        verbose!(
            "Created new `MatchPos` object with params: {:?}",
            (line, start, end)
        );
        Self { line, start, end }
    }
}

/// Retrieves the 1‑based line number, start byte offset, and end byte
/// offset for every regex match located within the given input string.
#[inline]
#[must_use = "Do not discard returned value"]
pub fn scan_matched(s: &str, pattern: &Regex) -> Vec<MatchPos> {
    verbose!("Scanning matches of regex {pattern}");
    let mut res: Vec<MatchPos> = Vec::new();
    for (line_idx, line) in s.lines().enumerate() {
        let line_no = line_idx + 1;
        if let Some(cap) = pattern.captures(line) {
            let mat = cap.get(0).unwrap(); // get(0) will always return Some(xxx)
            res.push(MatchPos::new(line_no, mat.start(), mat.end()));
        }
    }
    res
}

/// Print the error message.
#[inline]
pub fn print_msg(prompt: &str, filename: &str, pos: &MatchPos, line_text: &str) {
    eprintln!("{prompt}: file {filename}, line {}:", pos.line);
    eprintln!("{} | {line_text}", pos.line);
    eprintln!(
        "{}   {}{}{}",
        " ".repeat(pos.line.to_string().len()), // number of digits
        " ".repeat(pos.start),                  // previous spaces
        "^".repeat(pos.end - pos.start),        // underline error contents
        " ".repeat(line_text.len() - pos.end)   // rest spaces
    );
}

fn kind2str(kind: it) -> &'static str {
    match kind {
        it::Evil => "Found evil: ",
        it::Hardcoded => "Found hard-coded: ",
        it::Quality => "Found code quality: ",
        it::Secrets => "Found secrets: ",
        it::All => "Found issue: ",
    }
}

#[must_use = "Do not discard returned value"]
/// Traverse all given regex, get scanned error, print message and return number of messages.
pub fn diagnostic_by_regex(rule: &Rule, file_content: &str, filename: &str) -> i32 {
    let mut err_cnt = 0;
    for pattern in &rule.regex {
        let mat = scan_matched(file_content, pattern);
        for pos in mat {
            let line_text = file_content.lines().collect::<Vec<_>>()[pos.line - 1];
            // Print error message
            print_msg(
                format!("{}{}", kind2str(rule.kind), rule.prompt).as_str(),
                filename,
                &pos,
                line_text,
            );
            err_cnt += 1;
        }
    }
    err_cnt
}
