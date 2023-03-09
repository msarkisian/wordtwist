use std::{error::Error, fmt::Display};

use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use rusqlite::Connection;

pub struct UserID(pub usize);
// Couldn't get argon2's errors to play nice with anyhow
// may refactor this custom error type later
#[derive(Debug)]
pub struct HashError;

impl Display for HashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error hashing password")
    }
}
impl Error for HashError {}

pub fn add_user(
    conn: &mut Connection,
    username: &str,
    email: &str,
    password: &str,
) -> Result<UserID> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| HashError)?;

    let user_id = conn.query_row(
        "INSERT INTO users (email, username, password_hash) VALUES (?1, ?2, ?3) RETURNING id",
        (email, username, password_hash.to_string()),
        |r| Ok(UserID(r.get(0)?)),
    )?;
    Ok(user_id)
}
