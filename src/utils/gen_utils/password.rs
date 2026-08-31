//! Generates passwords.

use rand::{self, RngExt};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789\
                        !@#$%^&*-_";

/// Generates a secure password with given length.
pub fn gen_password(len: usize) -> String {
    let mut rng = rand::rng();
    let mut password = String::with_capacity(len);
    for _ in 0..len {
        let idx = rng.random_range(0..CHARSET.len());
        password.push(CHARSET[idx] as char);
    }
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
