use std::{error::Error, fmt::Display};

use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use rusqlite::Connection;

pub struct UserID(pub usize);

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
        .map_err(|e| anyhow!("error hashing password: {:?}", e))?;

    let user_id = conn.query_row(
        "INSERT INTO users (email, username, password_hash) VALUES (?1, ?2, ?3) RETURNING id",
        (email, username, password_hash.to_string()),
        |r| Ok(UserID(r.get(0)?)),
    )?;
    Ok(user_id)
}
