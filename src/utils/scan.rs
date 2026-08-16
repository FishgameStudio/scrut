//! Scan functions.

use std::{env, error, fs};

use globset::Glob;

use crate::utils::{logging::verbose, scan_utils::secrets::scan_secrets};

/// Check if path matches glob pattern, input path should be unix‑style relative path.
fn is_match(pattern: &str, rel_path: &str) -> bool {
    let glob = match Glob::new(pattern) {
        Ok(g) => g,
        Err(_) => return false,
    };
    glob.compile_matcher().is_match(rel_path)
}

/// Recursively scan files inside given directory.
/// Action receives absolute file path string.
pub fn scan_files(
    action: &impl Fn(&String) -> Result<(), Box<dyn error::Error>>,
    directory: &String,
) -> Result<(), Box<dyn error::Error>> {
    let entries = fs::read_dir(directory)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let full_path = path.to_str().ok_or("Invalid UTF‑8 file path")?.to_string();

        if path.is_dir() {
            verbose!("Recursing directory: {}", full_path);
            scan_files(action, &full_path)?;
        } else if path.is_file() {
            verbose!("Processing file: {}", full_path);
            action(&full_path)?;
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub enum ItemType {
    Secrets = 0x1,
    Evil = 0x2,
    All = 0x1 | 0x2,
}

/// Convert string argument to scan type enum.
pub fn str2enum(name: &str) -> Result<ItemType, Box<dyn error::Error>> {
    match name {
        "*" | "all" => Ok(ItemType::All),
        "evil" => Ok(ItemType::Evil),
        "secrets" => Ok(ItemType::Secrets),
        other => Err(format!("unknown scan item name: {}", other).into()),
    }
}

#[inline]
pub fn bit_mask(item: u64, target: ItemType) -> bool {
    (item & target as u64) != 0
}

/// Main scan entry, handle exclude patterns, .gitignore parsing and iterate scan roots.
pub fn scan_cwd(
    exclude: &[String],
    scan_all: bool,
    items: &[&String],
) -> Result<(), Box<dyn error::Error>> {
    let cwd = env::current_dir()?;
    let cwd_str = cwd
        .to_str()
        .ok_or("Current working directory has invalid UTF‑8 path")?;

    let mut scan_mask: u64 = 0;
    for it in items {
        let tp = str2enum(it)?;
        scan_mask |= tp as u64;
    }

    let mut exclude_patterns: Vec<String> = exclude.to_vec();
    let mut include_patterns: Vec<String> = Vec::new();

    if exclude_patterns.is_empty() && !scan_all {
        if fs::exists(".gitignore")? {
            let bytes = fs::read(".gitignore")?;
            match String::from_utf8(bytes) {
                Ok(content) => {
                    for line in content.lines() {
                        let line = line.trim();
                        if line.is_empty() || line.starts_with('#') {
                            continue;
                        }
                        if let Some(stripped) = line.strip_prefix('!') {
                            include_patterns.push(stripped.replace("\\", "/"));
                        } else {
                            exclude_patterns.push(line.replace("\\", "/"));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("warning: Failed to read .gitignore: {}", e);
                }
            }
        } else {
            verbose!(".gitignore not found, skip loading ignore rules");
        }
    }

    let action = |full_file_path: &String| -> Result<(), Box<dyn error::Error>> {
        let rel_path = full_file_path
            .strip_prefix(cwd_str)
            .ok_or("File path is outside working directory")?
            .replace("\\", "/");
        let rel_path = rel_path.strip_prefix('/').unwrap_or(&rel_path);

        let mut excluded = exclude_patterns.iter().any(|rule| is_match(rule, rel_path));
        if excluded {
            let forced_include = include_patterns.iter().any(|inc| is_match(inc, rel_path));
            if forced_include {
                excluded = false;
            }
        }

        if excluded {
            println!("File {} excluded", rel_path);
            verbose!("Skipped excluded file: {}", rel_path);
            return Ok(());
        }

        println!("Processing file: {}", rel_path);
        verbose!("Absolute file path: {}", full_file_path);

        let bytes = fs::read(full_file_path)?;
        match String::from_utf8(bytes) {
            Ok(text) => {
                use ItemType::*;
                if bit_mask(scan_mask, Secrets) {
                    verbose!("Running secrets scan on: {}", rel_path);
                    scan_secrets(&text, full_file_path);
                }
                if bit_mask(scan_mask, Evil) {
                    verbose!("Running evil‑pattern scan on: {}", rel_path);
                    todo!("Evil content scan not implemented");
                }
            }
            Err(e) => {
                eprintln!("warning: Skip non‑utf8 file {}: {}", rel_path, e);
            }
        }
        Ok(())
    };

    for scan_root in items {
        if !fs::exists(scan_root)? {
            eprintln!("error: Scan root path does not exist: {}", scan_root);
            continue;
        }
        verbose!("Start scanning root: {}", scan_root);
        scan_files(&action, scan_root)?;
    }

    Ok(())
}
