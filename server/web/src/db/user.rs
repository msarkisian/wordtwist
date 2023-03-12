use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
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

pub fn validate_user(conn: &mut Connection, username: &str, password: &str) -> Result<UserID> {
    let (id, password_hash) = conn.query_row(
        "SELECT id, password_hash FROM users WHERE username=?1",
        (username,),
        |r| Ok((r.get::<usize, usize>(0), r.get::<usize, String>(1))),
    )?;
    let password_hash = password_hash.unwrap_or_default();
    let parsed_hash =
        PasswordHash::new(&password_hash).map_err(|_| anyhow!("invalid username or password"))?;

    if Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(anyhow!("invalid username or password"));
    }

    Ok(UserID(id.unwrap()))
}
