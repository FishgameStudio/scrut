//! Scan functions.

use std::{env, error, fs};

use globset::Glob;

use crate::utils::logging::verbose;

use crate::utils::scan_utils::{
    evil::scan_evil, hardcoded::scan_hardcoded, quality::scan_quality, secrets::scan_secrets,
};

/// Determine whether a file matches a wildcard syntax.
fn is_match(pattern: &str, rel_path_unix: &str) -> bool {
    let glob_res = Glob::new(pattern);
    match glob_res {
        Ok(g) => g.compile_matcher().is_match(rel_path_unix),
        Err(_) => false,
    }
}

/// Recursion function to scan all files in specified directory.
pub fn scan_files(
    action: &mut impl FnMut(&String) -> Result<(), Box<dyn error::Error>>,
    directory: &String,
) -> Result<(), Box<dyn error::Error>> {
    let entries = fs::read_dir(directory)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let full_path = path.to_str().ok_or("invalid utf‑8 file path")?.to_string();

        if path.is_dir() {
            verbose!("Recursing directory: {}", full_path);
            scan_files(action, &full_path)?;
        } else if path.is_file() {
            verbose!("Doing action for file: {}", full_path);
            action(&full_path)?;
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub enum ItemType {
    Secrets = 0x1,
    Evil = 0x2,
    Hardcoded = 0x4,
    Quality = 0x8,
    All = 0xF,
}

/// Convert string name to ItemType
/// # Panics
/// If the name is not recongnized.
pub fn str2enum(name: &str) -> ItemType {
    use ItemType::*;
    match name {
        "*" | "all" => All,
        "evil" => Evil,
        "secrets" => Secrets,
        "hardcoded" => Hardcoded,
        "quality" => Quality,
        other => panic!("fatal: unknown item name: {}", other),
    }
}

#[inline]
pub fn bit_mask(item: u64, target: ItemType) -> bool {
    (item & target as u64) != 0
}

fn is_glob_meta(s: &str) -> bool {
    s.contains(&['*', '?', '[', ']'][..])
}

/// Recursion function to scan files in current working directory.
pub fn scan_cwd(
    user_exclude: &[String],
    scan_all: bool,
    items: &[&String],
) -> Result<(), Box<dyn error::Error>> {
    let cwd = env::current_dir()?;
    let cwd_str = cwd
        .to_str()
        .ok_or("current working directory contains invalid utf‑8")?;

    // Build scan mask by bit‑or multiple items
    let mut mask: u64 = 0x0;
    for it in items {
        let tp = str2enum(it);
        mask |= tp as u64;
    }

    let mut exclude_patterns: Vec<String> = user_exclude.to_vec();
    let mut include_patterns: Vec<String> = Vec::new();

    if scan_all {
        // scan‑all mode: keep user ‑e excludes, add .git hard exclude
        if !exclude_patterns.iter().any(|s| s == ".git") {
            exclude_patterns.push(".git".to_string());
            exclude_patterns.push(".git/**".to_string());
        }
    } else {
        // normal mode: load .gitignore
        if match fs::exists(".gitignore") {
            Ok(exists) => exists,
            Err(e) => {
                eprintln!("warning: failed to stat .gitignore: {}", e);
                false
            }
        } {
            let bytes = fs::read(".gitignore")?;
            match String::from_utf8(bytes) {
                Ok(content) => {
                    for line in content.lines() {
                        let line = line.trim();
                        if line.is_empty() || line.starts_with('#') {
                            continue;
                        }

                        if let Some(stripped) = line.strip_prefix('!') {
                            let pat = stripped.replace("\\", "/");
                            if pat.ends_with('/') {
                                include_patterns.push(pat.clone());
                                include_patterns.push(format!("{}**", pat));
                            } else {
                                include_patterns.push(pat.clone());
                                if !is_glob_meta(&pat) {
                                    include_patterns.push(format!("{}/**", pat));
                                }
                            }
                        } else {
                            let pat = line.replace("\\", "/");
                            if pat.ends_with('/') {
                                exclude_patterns.push(pat.clone());
                                exclude_patterns.push(format!("{}**", pat));
                            } else {
                                exclude_patterns.push(pat.clone());
                                if !is_glob_meta(&pat) {
                                    exclude_patterns.push(format!("{}/**", pat));
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("warning: Skipping reading .gitignore: {}", e);
                    verbose!("skipping reading .gitignore: {}", e);
                }
            }
        } else {
            verbose!(".gitignore not found");
        }
        // always hard‑exclude .git folder for normal scan
        if !exclude_patterns.iter().any(|s| s == ".git") {
            exclude_patterns.push(".git".to_string());
        }
    }
    exclude_patterns.push("target/**".to_string());
    exclude_patterns.push(".git/**".to_string());
    exclude_patterns.push("Cargo.*".to_string());
    exclude_patterns.push(".cargo/**".to_string());

    let mut issues: i32 = 0;
    //////// File Processor Closure ////////
    let mut action = |full_abs: &String| -> Result<(), Box<dyn std::error::Error>> {
        // convert absolute path to relative unix‑style path for glob matching
        let rel = full_abs
            .strip_prefix(cwd_str)
            .ok_or("file outside working directory")?
            .replace("\\", "/");
        let rel = rel.strip_prefix('/').unwrap_or(&rel);

        let mut is_excluded = exclude_patterns.iter().any(|rule| is_match(rule, rel));
        // gitignore ! override: if matches include pattern, cancel exclude
        if is_excluded {
            let forced_include = include_patterns
                .iter()
                .any(|inc_rule| is_match(inc_rule, rel));
            if forced_include {
                is_excluded = false;
            }
        }

        if is_excluded {
            verbose!("Skipped excluded file {}", rel);
            return Ok(());
        }

        verbose!("Processing absolute path: {}", full_abs);

        let bytes = fs::read(full_abs)?;

        match String::from_utf8(bytes) {
            Ok(text) => {
                use ItemType::*;
                if bit_mask(mask, Secrets) {
                    verbose!("Scanning secrets in file {}", rel);
                    issues += scan_secrets(&text, full_abs);
                }
                if bit_mask(mask, Evil) {
                    verbose!("Scanning evils in file {}", rel);
                    issues += scan_evil(&text, full_abs);
                }
                if bit_mask(mask, Hardcoded) {
                    verbose!("Scanning hard-coded in file {}", rel);
                    issues += scan_hardcoded(&text, full_abs);
                }
                if bit_mask(mask, Quality) {
                    verbose!("Scanning safety in file {}", rel);
                    issues += scan_quality(&text, full_abs);
                }
            }
            Err(e) => {
                verbose!("Skipping reading {}: {}", rel, e);
            }
        }
        Ok(())
    };

    scan_files(&mut action, &cwd_str.to_string())?;

    if issues > 0 {
        eprintln!("\nOh no!");
        eprintln!("{issues} issues found.");
        verbose!("{issues} issues found");
    } else {
        println!("No issues found!");
        verbose!("No issues found");
    }

    Ok(())
}
