//! Generates passwords.

use rand::{self, RngExt};

/// Generates a secure password with given length.
pub fn gen_password(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789\
                            !@#$%^&*-_";
    let mut rng = rand::rng();
    let mut password = String::with_capacity(len);
    for _ in 0..len {
        let idx = rng.random_range(0..CHARSET.len());
        password.push(CHARSET[idx] as char);
    }
    password
}
