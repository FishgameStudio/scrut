//! Scan personal informations and secrets in file contents.

use regex::Regex;

use crate::utils::logging::verbose;

use lazy_static::lazy_static;

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
pub fn scan_matched(s: &String, pattern: &Regex) -> Vec<MatchPos> {
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
pub fn print_msg(prompt: &str, filename: &String, pos: &MatchPos, line_text: &str) -> () {
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
/// Traverse all given regex, get scaned error, print message and return number of messages.
pub fn diagnostic(
    error_msg: &str,
    patterns: &Vec<Regex>,
    file_content: &String,
    filename: &String,
) -> i32 {
    let mut err_cnt = 0;
    for pattern in patterns {
        let mat = scan_matched(file_content, pattern);
        for pos in mat {
            let line_text = file_content.lines().collect::<Vec<_>>()[pos.line - 1];
            // Print error message
            print_msg(error_msg, filename, &pos, line_text);
            err_cnt += 1;
        }
    }
    err_cnt
}

lazy_static! {
    // API Token
    static ref TOKEN_RULES: Vec<Regex> = vec![
        Regex::new(r"\w+_pat_[a-zA-Z0-9_\-]{30,85}").unwrap(),
        Regex::new(r"Bearer\s+(.+)").unwrap(),
        Regex::new(r#"let token\s*=\s*"\w+_pat_[a-zA-Z0-9_\-]{30,85}"\s*;"#).unwrap(),
        Regex::new(r#"let token\s*=\s*"Bearer\s+(.+)"\s*;"#).unwrap(),
    ];
    // Password
    static ref PASSWORD_RULES: Vec<Regex> = vec![
        Regex::new(r#"let password\s*=".*""#).unwrap(),
        Regex::new(r#"(password|passwd)\s*[:=]\s*["'][^"']+["']"#).unwrap(),
    ];
    static ref OTHER_RULES: Vec<Regex> = vec![
        // IPv4 / IPv6
        Regex::new(r#"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b"#).unwrap(),
        Regex::new(r#"\b(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}|(?:[0-9a-fA-F]{1,4}:){1,7}:|(?:[0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|(?:[0-9a-fA-F]{1,4}:){1,5}(?::[0-9a-fA-F]{1,4}){1,2}|(?:[0-9a-fA-F]{1,4}:){1,4}(?::[0-9a-fA-F]{1,4}){1,3}|(?:[0-9a-fA-F]{1,4}:){1,3}(?::[0-9a-fA-F]{1,4}){1,4}|(?:[0-9a-fA-F]{1,4}:){1,2}(?::[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:(?::[0-9a-fA-F]{1,4}){1,6}|:(?::[0-9a-fA-F]{1,4}){1,7}|::(?:[0-9a-fA-F]{1,4}:){0,6}[0-9a-fA-F]{1,4}|[0-9a-fA-F]{1,4}::(?:[0-9a-fA-F]{1,4}:){0,5}[0-9a-fA-F]{1,4}\b"#).unwrap(),
        // URL embedded credentials
        Regex::new(r#"https?://[^"'`\s:@]+:[^"'`\s@]+@"#).unwrap(),
        // JWT
        Regex::new(r#"\beyJ[A-Za-z0-9_\-]+\.[A-Za-z0-9_\-]+\.[A-Za-z0-9_\-]+\b"#).unwrap(),
    ];
}

/// Scan personal informations and secrets in file contents.
pub fn scan_secrets(s: &String, filename: &String) -> i32 {
    let mut total_errors = 0;
    total_errors += diagnostic("Hard-coded token", &*TOKEN_RULES, s, filename);
    total_errors += diagnostic("Hard-coded password", &*PASSWORD_RULES, s, filename);
    total_errors += diagnostic("Found secret", &*OTHER_RULES, s, filename);
    total_errors
}
