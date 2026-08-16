//! Scan functions.

use std::{self, env, error, fs};

use globset::Glob;

use crate::utils::scan_utils::secrets::scan_secrets;

/// Determine whether a file matches a wildcard syntax.
fn is_match(pattern: &str, name: &str) -> bool {
    let glob_res = Glob::new(pattern);
    if let Err(_) = glob_res {
        false
    } else {
        glob_res.unwrap().compile_matcher().is_match(name)
    }
}

/// Recursion function to scan all files in specified directory.
pub fn scan_files(
    action: &impl Fn(&String) -> Result<(), Box<dyn error::Error>>,
    directory: &String,
) -> Result<(), Box<dyn error::Error>> {
    let entries = fs::read_dir(directory)?;

    for entry in entries {
        let entry = entry?;
        let oss_name: std::ffi::OsString = entry.file_name();
        let name = oss_name.to_string_lossy().into_owned();

        if entry.path().is_dir() {
            // Directory -> recursion.
            scan_files(action, &name)?;
        } else if entry.path().is_file() {
            // File -> do specified action.
            action(&name)?;
        }
    }

    Ok(())
}

#[derive(Debug)]
pub enum ItemType {
    Secrets = 0x1,
    Evil = 0x2,
    All = 0x1 | 0x2,
}

/// Match the list and convert to enum
/// # Panics
/// If item name does not match anyone enumerate.
fn str2enum(name: &str) -> ItemType {
    match name {
        "*" | "all" => ItemType::All,
        "evil" => ItemType::Evil,
        "secrets" => ItemType::Secrets,
        other => panic!("fatal: unknown item name: {}", other),
    }
}

#[inline]
fn bit_mask(item: u64, target: ItemType) -> bool {
    (item & target as u64) != 0
}

/// Recursion function to scan files in current working directory.
pub fn scan_cwd(
    exclude: &Vec<String>,
    scan_all: bool,
    items: &Vec<&String>,
) -> Result<(), Box<dyn error::Error>> {
    let cwd = env::current_dir()?;
    // Bit or
    let mut item: u64 = 0x0;
    for it in items {
        item |= str2enum(it) as u64;
    }

    let mut exclude: Vec<String> = exclude.clone();
    let mut include: Vec<String> = Vec::new();
    if exclude.is_empty() && !scan_all {
        // Automatically generate excluding list by .gitignore.
        // Read .gitignore if exists.
        if fs::exists(".gitignore").is_err() {
            eprintln!("warning: Skipping reading .gitignore because it doesn't exist.");
        } else {
            let bytes = fs::read(".gitignore")?;
            match String::from_utf8(bytes) {
                Ok(s) => {
                    let s = s.to_owned();
                    // Traverse each line and skip comments.
                    let lines = s.lines();
                    for line in lines {
                        let line = line.trim();
                        if line.starts_with("#") {
                            continue;
                        } else if line.starts_with("!") {
                            include.push(line.strip_prefix("!").unwrap().to_string());
                        }
                        exclude.push(line.replace("\\", "/").to_string());
                    }
                }
                Err(e) => {
                    eprintln!("warning: Skipping reading .gitignore: {}", e);
                }
            }
        }
    }

    //////// Main File Processer //////
    let action = |file: &String| -> Result<(), Box<dyn std::error::Error>> {
        // Check whether the file matches any pattern in the exclusion list.
        let is_excluded = exclude.iter().any(|rule| is_match(rule, file));
        if is_excluded && !include.contains(file) {
            println!("File {} excluded", file);
        } else {
            println!("Processing file {} with items {:?}", file, items);

            // Read file contents
            let bytes = fs::read(file)?;
            match String::from_utf8(bytes) {
                Ok(s) => {
                    let s = s.to_owned();
                    // Match item
                    use ItemType::*;
                    if bit_mask(item, Secrets) {
                        // API Tokens, passwords, etc.
                        scan_secrets(&s, file);
                    } else if bit_mask(item, Evil) {
                        // Evil scripts, logics, etc.
                        todo!("No implementations of evil content scan.");
                    }
                }
                Err(e) => {
                    eprintln!("warning: Skipping reading {}: {}", file, e);
                }
            }
        }
        Ok(())
    };

    scan_files(&action, &cwd.to_string_lossy().to_string())
}
