use rusqlite::{Connection, Result};

pub fn open_db_connection() -> Result<()> {
    let db = Connection::open("./web/sql.db3")?;

    println!("{}", db.is_autocommit());
    Ok(())
}
