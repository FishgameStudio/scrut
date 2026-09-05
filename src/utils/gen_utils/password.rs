//! Generates passwords.

use rand::{self, RngExt};

use crate::utils::logging::verbose;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789\
                        !@#$%^&*-_";

/// Generates a secure password with given length.
pub fn gen_password(len: usize) -> String {
    verbose!("Generating password with specified length {len} ...");
    let mut password = String::with_capacity(len);
    for _ in 0..len {
        let idx = rand::rng().random_range(0..CHARSET.len());
        password.push(CHARSET[idx] as char);
    }
    verbose!("Done");
    password
}

/// Unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn password_test() {
        use super::{CHARSET, gen_password};
        const LEN: usize = 10;
        let password = gen_password(LEN);
        assert_eq!(password.len(), LEN);
        assert!(password.bytes().any(|ch| CHARSET.contains(&ch)));
    }
}
