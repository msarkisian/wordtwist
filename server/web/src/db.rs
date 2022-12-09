use rusqlite::{Connection, Result};

mod game;

pub fn open_db_connection() -> Result<Connection> {
    Ok(Connection::open("../web/sql.db3")?)
}
