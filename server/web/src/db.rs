use rusqlite::Connection;

pub mod game;

pub fn open_db_connection() -> Connection {
    Connection::open("./web/sql.db3").expect("error connecting to database")
}
