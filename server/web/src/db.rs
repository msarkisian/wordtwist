use rusqlite::Connection;

pub mod game;
pub mod score;
pub mod user;

#[cfg(test)]
const DB_PATH: &str = "./sql.db3";
#[cfg(not(test))]
const DB_PATH: &str = "./server/web/sql.db3";
const FALLBACK_DB_PATH: &str = "./web/sql.db3";

pub fn open_db_connection() -> Connection {
    Connection::open(DB_PATH).unwrap_or_else(|_| {
        Connection::open(FALLBACK_DB_PATH).expect("error connecting to database")
    })
}
