use rusqlite::Connection;

pub mod game;

#[cfg(test)]
const DB_PATH: &'static str = "./sql.db3";
#[cfg(not(test))]
const DB_PATH: &'static str = ".server/web/sql.db3";
const FALLBACK_DB_PATH: &'static str = "./web/sql.db3";

pub fn open_db_connection() -> Connection {
    Connection::open(DB_PATH).unwrap_or_else(|_| {
        Connection::open(FALLBACK_DB_PATH).expect("error connecting to database")
    })
}
