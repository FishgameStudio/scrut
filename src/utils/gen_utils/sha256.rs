//! Generates SHA256 from specified contents.

use hex;

use sha2::{Digest, Sha256};

use std::error::Error;
use std::fs::read;
use std::path::PathBuf;

use crate::utils::logging::{verbose, warning};

use glob::glob;

/// Generate SHA256 from a specified content.
pub fn gen_sha256(content: &str) -> String {
    verbose!("Generating SHA256 of content {content:#?} ...");
    let mut hasher = Sha256::new();
    hasher.update(content);
    let res = hasher.finalize();
    verbose!("Done");
    hex::encode(res)
}

/// Glob patterns to generate checksums.
pub const CHECKSUM_GLOBS: [&str; 6] = [
    "**/*.tar.gz",
    "**/*.zip",
    "**/*.7z",
    "**/*.tar.xz",
    "**/*.exe",
    "**/*.elf",
];

/// Generate checksums (SHA256) for binary files.
/// # Errors
/// If failed to read content of file.
pub fn gen_checksums() -> Result<String, Box<dyn Error>> {
    let mut files: Vec<PathBuf> = Vec::new();
    for pattern in CHECKSUM_GLOBS {
        for entry in glob(pattern)? {
            let entry = entry?;
            files.push(entry);
        }
    }

    let mut checksum = "".to_string();
    let mut hasher = Sha256::new();
    for file in &files {
        let file = file.strip_prefix("./").unwrap_or(file);
        verbose!("Generating checksum of file");
        if !file.is_file() {
            warning!("Object '{}' is not a valid file, skipping", file.display());
            continue;
        }

        let bytes = read(file)?;
        hasher.update(bytes);
        let res = hasher.finalize_reset();
        let hash = hex::encode(res);

        checksum.push_str(&format!("{}  {}\n", hash, file.display()));
        verbose!("Hash pushed to checksum (was {hash})");
    }
    Ok(checksum)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sha() {
        use super::gen_sha256;
        let hash1 = gen_sha256("hello, world!");
        let hash2 = gen_sha256("SHA256 Unit Tests");
        assert_ne!(hash1, hash2);
        assert_eq!(hash1.len(), 64);
        assert_eq!(hash2.len(), 64);
    }
}
